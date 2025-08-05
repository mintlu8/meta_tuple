use crate::{Join, MetaItem, MetaTuple};
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

/// Erased [`MetaTuple`].
pub enum ErasedInnerPtr<'t> {
    None,
    Any(&'t dyn Any),
    Joined(&'t dyn MetaBox, &'t dyn MetaBox),
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
pub unsafe trait MetaBox {
    fn as_erased(&self) -> ErasedInner<'_>;
    fn as_erased_mut(&mut self) -> ErasedInnerMut<'_>;
    fn as_erased_ptr(&self) -> ErasedInnerPtr<'_>;
}

use crate::impl_meta_box;
impl_meta_box!(MetaBox);

/// Add the `get` and `get_mut` function on subtraits of [`Metabox`].
///
/// Syntax
///
/// ```
/// # use meta_tuple::*;
/// trait Metadata: MetaBox + 'static {}
///
/// impl_meta_box!(Metadata);
/// ```
#[macro_export]
macro_rules! impl_meta_box {
    ($trait: ident) => {
        const _: () = {
            use $crate::MetaBox;
            impl dyn $trait + '_ {
                /// Obtain an item if it exists in the [`MetaBox`].
                ///
                /// Always returns `Some` for `()`.
                pub fn get<T: 'static>(&self) -> Option<&T> {
                    if let Some(value) = (&() as &dyn $crate::Any).downcast_ref() {
                        return Some(value);
                    }
                    match self.as_erased() {
                        $crate::ErasedInner::None => None,
                        $crate::ErasedInner::Any(any) => any.downcast_ref(),
                        $crate::ErasedInner::Joined(a, b) => a.get().or_else(|| b.get()),
                    }
                }

                /// Obtain an item if it exists in the [`MetaBox`].
                ///
                /// Always returns `Some` for `()`.
                pub fn get_mut<T: 'static>(&mut self) -> Option<&mut T> {
                    if (&mut () as &mut dyn $crate::Any)
                        .downcast_mut::<T>()
                        .is_some()
                    {
                        // Safety:
                        //
                        // Safe since `()` is a ZST.
                        return Some(unsafe { ::core::ptr::NonNull::dangling().as_mut() });
                    }
                    match self.as_erased_mut() {
                        $crate::ErasedInnerMut::None => None,
                        $crate::ErasedInnerMut::Any(any) => any.downcast_mut(),
                        $crate::ErasedInnerMut::Joined(a, b) => a.get_mut().or_else(|| b.get_mut()),
                    }
                }
            }
        };
    };
}

impl dyn MetaBox + '_ {
    /// Obtain an item if it exists in the [`MetaBox`].
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
        }
    }
}

unsafe impl MetaBox for () {
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

unsafe impl<T: MetaBox> MetaBox for &T {
    fn as_erased<'t>(&self) -> ErasedInner<'_> {
        MetaBox::as_erased(*self)
    }

    fn as_erased_mut(&mut self) -> ErasedInnerMut<'_> {
        ErasedInnerMut::None
    }

    fn as_erased_ptr(&self) -> ErasedInnerPtr<'_> {
        ErasedInnerPtr::None
    }
}

unsafe impl<T: MetaBox> MetaBox for &mut T {
    fn as_erased<'t>(&self) -> ErasedInner<'_> {
        MetaBox::as_erased(*self)
    }

    fn as_erased_mut(&mut self) -> ErasedInnerMut<'_> {
        MetaBox::as_erased_mut(*self)
    }

    fn as_erased_ptr(&self) -> ErasedInnerPtr<'_> {
        MetaBox::as_erased_ptr(*self)
    }
}

unsafe impl<T: 'static> MetaBox for MetaItem<T> {
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

unsafe impl<T: 'static> MetaBox for Option<T> {
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

unsafe impl<A: MetaTuple, B: MetaTuple> MetaBox for Join<A, B> {
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
