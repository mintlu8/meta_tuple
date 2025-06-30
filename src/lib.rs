#![no_std]
mod dynamic;
mod into;
mod item;
mod r#macro;

#[doc(hidden)]
pub use core::any::Any;
pub use dynamic::*;
pub use into::IntoMetaTuple;
pub use item::MetaItem;

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
///
/// `&impl MetaTuple` and `&mut impl MetaTuple` both implement MetaTuple,
/// for a tuple like `(&A, A)`, `get` returns the first value while `get_mut` returns the second value.
///
/// # Dyn Compatibility
///
/// For a boxed dynamic version, see super trait [`MetaBox`].
pub trait MetaTuple: MetaBox {
    /// Obtain an item, if exists.
    fn get<T: 'static>(&self) -> Option<&T>;
    /// Obtain a mutable item, if exists.
    fn get_mut<T: 'static>(&mut self) -> Option<&mut T>;

    /// Join with another concrete value.
    fn join<T: 'static>(self, other: T) -> (Self, MetaItem<T>)
    where
        Self: Sized,
    {
        (self, MetaItem(other))
    }

    /// Join with a reference to a concrete value.
    ///
    /// If querying for a mutable reference, will return `None`.
    fn join_ref<T: 'static>(self, other: &T) -> (Self, &T)
    where
        Self: Sized,
    {
        (self, other)
    }

    /// Join with a mutable reference to a concrete value.
    fn join_mut<T: 'static>(self, other: &mut T) -> (Self, &mut T)
    where
        Self: Sized,
    {
        (self, other)
    }

    /// Join with another [`MetaTuple`].
    fn join_tuple<T: MetaTuple>(self, other: T) -> (Self, T)
    where
        Self: Sized,
    {
        (self, other)
    }
}

impl<T: MetaTuple> MetaTuple for &T {
    fn get<U: 'static>(&self) -> Option<&U> {
        MetaTuple::get(*self)
    }

    fn get_mut<U: 'static>(&mut self) -> Option<&mut U> {
        None
    }
}

impl<T: MetaTuple> MetaTuple for &mut T {
    fn get<U: 'static>(&self) -> Option<&U> {
        MetaTuple::get(*self)
    }

    fn get_mut<U: 'static>(&mut self) -> Option<&mut U> {
        MetaTuple::get_mut(*self)
    }
}

impl MetaTuple for () {
    fn get<T: 'static>(&self) -> Option<&T> {
        None
    }

    fn get_mut<T: 'static>(&mut self) -> Option<&mut T> {
        None
    }
}

impl<T: 'static> MetaTuple for MetaItem<T> {
    fn get<U: 'static>(&self) -> Option<&U> {
        (&self.0 as &dyn Any).downcast_ref()
    }

    fn get_mut<U: 'static>(&mut self) -> Option<&mut U> {
        (&mut self.0 as &mut dyn Any).downcast_mut()
    }
}

impl<T: 'static> MetaTuple for Option<T> {
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
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Join<A, B>(pub A, pub B);

impl<A: MetaTuple, B: MetaTuple> MetaTuple for Join<A, B> {
    fn get<T: 'static>(&self) -> Option<&T> {
        self.0.get().or_else(|| self.1.get())
    }

    fn get_mut<T: 'static>(&mut self) -> Option<&mut T> {
        self.0.get_mut().or_else(|| self.1.get_mut())
    }
}
