use crate::{MetaItem, MetaTuple};
use core::{any::Any, ptr::NonNull};

/// Erased [`MetaTuple`].
pub enum ErasedInner<'t> {
    None,
    Any(&'t dyn Any),
    Joined(&'t dyn MetaBox, &'t dyn MetaBox),
}

/// Erased mutable [`MetaTuple`].
pub enum ErasedInnerMut<'t> {
    None,
    Any(&'t mut dyn Any),
    Joined(&'t mut dyn MetaBox, &'t mut dyn MetaBox),
}

/// A dyn compatible alternative to [`Any`] that can contain multiple items.
pub trait MetaBox {
    fn as_erased(&self) -> ErasedInner<'_>;
    fn as_erased_mut(&mut self) -> ErasedInnerMut<'_>;
}

impl dyn MetaBox + '_ {
    /// Obtain an item if it exists in the [`MetaBox`].
    ///
    /// Always returns `Some` for `()`.
    pub fn get<T: 'static>(&self) -> Option<&T> {
        if let Some(value) = (&() as &dyn Any).downcast_ref() {
            return Some(value);
        }
        match self.as_erased() {
            ErasedInner::None => None,
            ErasedInner::Any(any) => any.downcast_ref(),
            ErasedInner::Joined(a, b) => a.get().or_else(|| b.get()),
        }
    }

    /// Obtain an item if it exists in the [`MetaBox`].
    ///
    /// Always returns `Some` for `()`.
    pub fn get_mut<T: 'static>(&mut self) -> Option<&mut T> {
        if (&mut () as &mut dyn Any).downcast_mut::<T>().is_some() {
            // Safety:
            //
            // Safe since `()` is a ZST.
            return Some(unsafe { NonNull::dangling().as_mut() });
        }
        match self.as_erased_mut() {
            ErasedInnerMut::None => None,
            ErasedInnerMut::Any(any) => any.downcast_mut(),
            ErasedInnerMut::Joined(a, b) => a.get_mut().or_else(|| b.get_mut()),
        }
    }
}

impl MetaBox for () {
    fn as_erased<'t>(&self) -> ErasedInner<'_> {
        ErasedInner::None
    }

    fn as_erased_mut(&mut self) -> ErasedInnerMut<'_> {
        ErasedInnerMut::None
    }
}

impl<T: MetaBox> MetaBox for &T {
    fn as_erased<'t>(&self) -> ErasedInner<'_> {
        MetaBox::as_erased(*self)
    }

    fn as_erased_mut(&mut self) -> ErasedInnerMut<'_> {
        ErasedInnerMut::None
    }
}

impl<T: MetaBox> MetaBox for &mut T {
    fn as_erased<'t>(&self) -> ErasedInner<'_> {
        MetaBox::as_erased(*self)
    }

    fn as_erased_mut(&mut self) -> ErasedInnerMut<'_> {
        MetaBox::as_erased_mut(*self)
    }
}

impl<T: 'static> MetaBox for MetaItem<T> {
    fn as_erased<'t>(&self) -> ErasedInner<'_> {
        ErasedInner::Any(&self.0)
    }

    fn as_erased_mut(&mut self) -> ErasedInnerMut<'_> {
        ErasedInnerMut::Any(&mut self.0)
    }
}

impl<T: 'static> MetaBox for Option<T> {
    fn as_erased<'t>(&self) -> ErasedInner<'_> {
        match self.as_ref() {
            Some(value) => ErasedInner::Any(value),
            None => ErasedInner::None,
        }
    }

    fn as_erased_mut(&mut self) -> ErasedInnerMut<'_> {
        match self.as_mut() {
            Some(value) => ErasedInnerMut::Any(value),
            None => ErasedInnerMut::None,
        }
    }
}

impl<A: MetaTuple, B: MetaTuple> MetaBox for (A, B) {
    fn as_erased<'t>(&self) -> ErasedInner<'_> {
        ErasedInner::Joined(&self.0, &self.1)
    }

    fn as_erased_mut(&mut self) -> ErasedInnerMut<'_> {
        ErasedInnerMut::Joined(&mut self.0, &mut self.1)
    }
}
