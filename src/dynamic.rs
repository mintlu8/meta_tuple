use crate::{Join, MetaItem, MetaTuple};
use core::{any::Any, ptr::NonNull};

/// Utility trait for erasing structs.
pub trait MetaBundle {
    fn get_field(&self, idx: usize) -> Option<&dyn Any>;
    fn get_field_mut(&mut self, idx: usize) -> Option<&mut dyn Any>;
}

/// Erased [`MetaTuple`].
pub enum ErasedInner<'t> {
    None,
    Any(&'t dyn Any),
    Joined(&'t dyn MetaAny, &'t dyn MetaAny),
    Struct(&'t dyn MetaBundle),
}

/// Erased mutable [`MetaTuple`].
pub enum ErasedInnerMut<'t> {
    None,
    Any(&'t mut dyn Any),
    Joined(&'t mut dyn MetaAny, &'t mut dyn MetaAny),
    Struct(&'t mut dyn MetaBundle),
}

/// Erased [`MetaTuple`] that returns pointers.
pub enum ErasedInnerPtr<'t> {
    None,
    Any(&'t dyn Any),
    Joined(&'t dyn MetaAny, &'t dyn MetaAny),
    Struct(&'t dyn MetaBundle),
}

/// A dyn compatible alternative to [`Any`] that can contain multiple items.
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
pub unsafe trait MetaAny {
    fn as_erased(&self) -> ErasedInner<'_>;
    fn as_erased_mut(&mut self) -> ErasedInnerMut<'_>;
    fn as_erased_ptr(&self) -> ErasedInnerPtr<'_>;
}

use crate::impl_meta_any;
impl_meta_any!(MetaAny);

impl dyn MetaAny + '_ {
    /// Obtain an item if it exists in the [`MetaAny`].
    ///
    /// Always returns `Some` for `()`.
    pub fn get_ptr<T: 'static>(&self) -> Option<*mut T> {
        if (&() as &dyn Any).downcast_ref::<()>().is_some() {
            return Some(NonNull::dangling().as_ptr());
        }
        match self.as_erased_ptr() {
            ErasedInnerPtr::None => None,
            ErasedInnerPtr::Any(any) => any.downcast_ref().map(|x| x as *const T as *mut T),
            ErasedInnerPtr::Joined(a, b) => a.get_ptr().or_else(|| b.get_ptr()),
            ErasedInnerPtr::Struct(s) => {
                let mut idx = 0;
                while let Some(field) = s.get_field(idx) {
                    if let Some(result) = field.downcast_ref() {
                        return Some(result as *const T as *mut T);
                    }
                    idx += 1;
                }
                None
            }
        }
    }
}

unsafe impl MetaAny for () {
    fn as_erased<'t>(&self) -> ErasedInner<'_> {
        ErasedInner::None
    }

    fn as_erased_mut(&mut self) -> ErasedInnerMut<'_> {
        ErasedInnerMut::None
    }

    fn as_erased_ptr(&self) -> ErasedInnerPtr<'_> {
        ErasedInnerPtr::None
    }
}

unsafe impl<T: MetaAny> MetaAny for &T {
    fn as_erased<'t>(&self) -> ErasedInner<'_> {
        MetaAny::as_erased(*self)
    }

    fn as_erased_mut(&mut self) -> ErasedInnerMut<'_> {
        ErasedInnerMut::None
    }

    fn as_erased_ptr(&self) -> ErasedInnerPtr<'_> {
        ErasedInnerPtr::None
    }
}

unsafe impl<T: MetaAny> MetaAny for &mut T {
    fn as_erased<'t>(&self) -> ErasedInner<'_> {
        MetaAny::as_erased(*self)
    }

    fn as_erased_mut(&mut self) -> ErasedInnerMut<'_> {
        MetaAny::as_erased_mut(*self)
    }

    fn as_erased_ptr(&self) -> ErasedInnerPtr<'_> {
        MetaAny::as_erased_ptr(*self)
    }
}

unsafe impl<T: 'static> MetaAny for MetaItem<T> {
    fn as_erased<'t>(&self) -> ErasedInner<'_> {
        ErasedInner::Any(&self.0)
    }

    fn as_erased_mut(&mut self) -> ErasedInnerMut<'_> {
        ErasedInnerMut::Any(&mut self.0)
    }

    fn as_erased_ptr(&self) -> ErasedInnerPtr<'_> {
        ErasedInnerPtr::Any(&self.0)
    }
}

unsafe impl<T: 'static> MetaAny for Option<T> {
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

    fn as_erased_ptr(&self) -> ErasedInnerPtr<'_> {
        match self.as_ref() {
            Some(value) => ErasedInnerPtr::Any(value),
            None => ErasedInnerPtr::None,
        }
    }
}

unsafe impl<A: MetaTuple, B: MetaTuple> MetaAny for Join<A, B> {
    fn as_erased<'t>(&self) -> ErasedInner<'_> {
        ErasedInner::Joined(&self.0, &self.1)
    }

    fn as_erased_mut(&mut self) -> ErasedInnerMut<'_> {
        ErasedInnerMut::Joined(&mut self.0, &mut self.1)
    }

    fn as_erased_ptr(&self) -> ErasedInnerPtr<'_> {
        ErasedInnerPtr::Joined(&self.0, &self.1)
    }
}
