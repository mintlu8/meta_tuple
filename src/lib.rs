#![no_std]
mod dynamic;
mod into;
mod item;
mod r#macro;
mod query;

#[doc(hidden)]
pub use core::any::Any;
pub use dynamic::*;
pub use into::IntoMetaTuple;
pub use item::MetaItem;
pub use query::MetaQuery;
#[doc(hidden)]
pub use query::MetaQuerySingle;

#[cfg(feature = "derive")]
pub use meta_tuple_derive::{MetaItem, MetaQuery, MetaTuple};

pub(crate) use polonius_the_crab::{polonius_loop, polonius_return};

/// A statically typed opaque tuple that can contain any type.
///
/// This is a zero cost abstraction in most cases. To create, see macro [`meta_tuple!`].
///
/// # Use Case
///
/// This trait is generally used for complicated input and output abstractions, for example
///
/// ```
/// pub trait CardComponent {
///     fn play(&self, input: &impl MetaTuple) -> impl MetaTuple;
/// }
///
/// impl CardComponent for Attack {
///     fn play(&self, input: &impl MetaTuple) -> impl MetaTuple {
///         let attacker = input.get::<Attacker>().unwrap();
///         let defender = input.get::<Defender>().unwrap();
///         let damage_dealt = self.calculate_damage(attacker, defender);
///         input.join(DamageDealt(damage_dealt))
///     }
/// }
///
/// pub trait CardComponent {
///     pub fn play(&self, input: &impl MetaTuple) -> impl MetaTuple;
/// }
/// ```
///
/// # Semantics
///
/// For functions like `get`, we look for the first correct item, duplicated items will not be used.
/// `&impl MetaTuple` and `&mut impl MetaTuple` both implement MetaTuple.
///
/// ## Warning
/// Due to our semantics, for a tuple like `(&A, A)`,
/// `get` returns the first value while `get_mut` returns the second value,
/// since `&A` cannot return a `&mut A`.
///
/// # Dyn Compatibility
///
/// For a boxed dynamic version, see super trait [`MetaAny`].
///
/// # Safety
///
/// This trait cannot return overlapping references for different types.
///
/// For example
///
/// ```
/// struct A{
///     b: B,
///     c: C,
/// }
/// ```
///
/// The implementation can return `A`, or `(B, C)` but not both.
pub unsafe trait MetaTuple: MetaAny {
    /// Obtain an item, if exists.
    fn get<T: 'static>(&self) -> Option<&T>;
    /// Obtain a mutable item, if exists.
    fn get_mut<T: 'static>(&mut self) -> Option<&mut T>;
    /// Obtain a mutable item as pointer, if exists.
    fn get_mut_ptr<T: 'static>(&self) -> Option<*mut T>;

    /// Join with another concrete value.
    fn join<T: 'static>(self, other: T) -> Join<Self, MetaItem<T>>
    where
        Self: Sized,
    {
        Join(self, MetaItem(other))
    }

    /// Join with a reference to a concrete value.
    ///
    /// If querying for a mutable reference, will return `None`.
    fn join_ref<T: 'static>(self, other: &T) -> Join<Self, &MetaItem<T>>
    where
        Self: Sized,
    {
        Join(self, MetaItem::from_ref(other))
    }

    /// Join with a mutable reference to a concrete value.
    fn join_mut<T: 'static>(self, other: &mut T) -> Join<Self, &mut MetaItem<T>>
    where
        Self: Sized,
    {
        Join(self, MetaItem::from_mut(other))
    }

    /// Join with another [`MetaTuple`].
    fn join_tuple<T: MetaTuple>(self, other: T) -> Join<Self, T>
    where
        Self: Sized,
    {
        Join(self, other)
    }

    /// Try obtain multiple values from the [`MetaTuple`].
    fn query_ref<T: MetaQuery>(&self) -> Option<T::Output<'_>> {
        T::query_ref(self)
    }

    /// Try obtain multiple values from the [`MetaTuple`].
    fn query_mut<T: MetaQuery>(&mut self) -> Option<T::Output<'_>> {
        T::query_mut(self)
    }
}

unsafe impl<T: MetaTuple> MetaTuple for &T {
    fn get<U: 'static>(&self) -> Option<&U> {
        MetaTuple::get(*self)
    }

    fn get_mut<U: 'static>(&mut self) -> Option<&mut U> {
        None
    }

    fn get_mut_ptr<U: 'static>(&self) -> Option<*mut U> {
        None
    }
}

unsafe impl<T: MetaTuple> MetaTuple for &mut T {
    fn get<U: 'static>(&self) -> Option<&U> {
        MetaTuple::get(*self)
    }

    fn get_mut<U: 'static>(&mut self) -> Option<&mut U> {
        MetaTuple::get_mut(*self)
    }

    fn get_mut_ptr<U: 'static>(&self) -> Option<*mut U> {
        MetaTuple::get_mut_ptr(*self)
    }
}

unsafe impl MetaTuple for () {
    fn get<T: 'static>(&self) -> Option<&T> {
        None
    }

    fn get_mut<T: 'static>(&mut self) -> Option<&mut T> {
        None
    }

    fn get_mut_ptr<T: 'static>(&self) -> Option<*mut T> {
        None
    }
}

unsafe impl<T: 'static> MetaTuple for MetaItem<T> {
    fn get<U: 'static>(&self) -> Option<&U> {
        (&self.0 as &dyn Any).downcast_ref()
    }

    fn get_mut<U: 'static>(&mut self) -> Option<&mut U> {
        (&mut self.0 as &mut dyn Any).downcast_mut()
    }

    fn get_mut_ptr<U: 'static>(&self) -> Option<*mut U> {
        (&self.0 as &dyn Any)
            .downcast_ref()
            .map(|x| x as *const U as *mut U)
    }
}

unsafe impl<T: 'static> MetaTuple for Option<T> {
    fn get<U: 'static>(&self) -> Option<&U> {
        if let Some(v) = self.as_ref() {
            if let Some(result) = (v as &dyn Any).downcast_ref() {
                return Some(result);
            }
        }
        None
    }

    fn get_mut<U: 'static>(&mut self) -> Option<&mut U> {
        if let Some(v) = self.as_mut() {
            if let Some(result) = (v as &mut dyn Any).downcast_mut() {
                return Some(result);
            }
        }
        None
    }

    fn get_mut_ptr<U: 'static>(&self) -> Option<*mut U> {
        if let Some(v) = self.as_ref() {
            if let Some(result) = (v as &dyn Any).downcast_ref() {
                return Some(result as *const U as *mut U);
            }
        }
        None
    }
}

/// Joins 2 [`MetaTuple`]s.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Join<A, B>(pub A, pub B);

unsafe impl<A: MetaTuple, B: MetaTuple> MetaTuple for Join<A, B> {
    fn get<T: 'static>(&self) -> Option<&T> {
        self.0.get().or_else(|| self.1.get())
    }

    fn get_mut<T: 'static>(&mut self) -> Option<&mut T> {
        self.0.get_mut().or_else(|| self.1.get_mut())
    }

    fn get_mut_ptr<T: 'static>(&self) -> Option<*mut T> {
        self.0.get_mut_ptr().or_else(|| self.1.get_mut_ptr())
    }
}
