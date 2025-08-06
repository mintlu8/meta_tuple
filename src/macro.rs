#[allow(unused)]
use crate::{IntoMetaTuple, MetaAny, MetaItem, MetaTuple};

/// Create a [`MetaTuple`].
///
/// # Syntax
///
/// To create a tuple like [`MetaTuple`]:
///
/// ```
/// meta_tuple!(1, 2.0, "hello", vec![1, 2])
/// ```
///
/// You can use `&` and `&mut`, to create non-static [`MetaTuple`]s,
/// keep in mind these are handled at the syntax level and not the type level.
///
/// ```
/// let a = 1;
/// let b = true
/// meta_tuple!(&a, &mut b);
/// ```
///
/// If already given a reference `x: &T`, using `x` is not allowed as the type of
/// `x` is not static, use `&*x` instead.
///
/// ```
/// let a = &1;
/// let b = &mut true
/// meta_tuple!(&*a, &mut *b);
/// ```
///
/// To join types that are already [`MetaTuple`]s, denote with a `#`.
///
/// ```
/// let a = meta_tuple!(1, 2.0);
/// let b = meta_tuple!(true);
/// let c = meta_tuple!(#a, #b, "hello");
/// ```
#[macro_export]
macro_rules! meta_tuple {
    () => {()};
    (@[$prev: expr]) => {
        $prev
    };
    (@[$prev: expr] #$e: expr $(, $($rest: tt)*)?) => {
        meta_tuple!(@[$crate::Join($prev, $e)] $($($rest)*)?)
    };
    (@[$prev: expr] &mut $e: expr $(, $($rest: tt)*)?) => {
        meta_tuple!(@[$crate::Join($prev, $crate::MetaItem::from_mut(&mut $e))] $($($rest)*)?)
    };
    (@[$prev: expr] &$e: expr $(, $($rest: tt)*)?) => {
        meta_tuple!(@[$crate::Join($prev, $crate::MetaItem::from_ref(&$e))] $($($rest)*)?)
    };
    (@[$prev: expr] $e: expr $(, $($rest: tt)*)?) => {
        meta_tuple!(@[$crate::Join($prev, $crate::MetaItem($e))] $($($rest)*)?)
    };
    (#$e: expr $(, $($rest: tt)*)?) => {
        meta_tuple!(@[$e] $($($rest)*)?)
    };
    (&mut $e: expr $(, $($rest: tt)*)?) => {
        meta_tuple!(@[$crate::MetaItem::from_mut(&mut $e)] $($($rest)*)?)
    };
    (&$e: expr $(, $($rest: tt)*)?) => {
        meta_tuple!(@[$crate::MetaItem::from_ref(&$e)] $($($rest)*)?)
    };
    ($e: expr $(, $($rest: tt)*)?) => {
        meta_tuple!(@[$crate::MetaItem($e)] $($($rest)*)?)
    };
}

/// Creates the typing of a [`MetaTuple`].
#[macro_export]
macro_rules! meta_tuple_type {
    () => {()};
    (@[$prev: ty]) => {
        $prev
    };
    (@[$prev: ty] #$ty: ty $(, $($tt:tt)*)?) => {
        $crate::meta_tuple_type!{@[$crate::Join<$prev, $ty>] $($($tt)*)?}
    };
    (@[$prev: ty] &mut $ty: ty $(, $($tt:tt)*)?) => {
        $crate::meta_tuple_type!{@[$crate::Join<$prev, &mut $crate::MetaItem<$ty>>] $($($tt)*)?}
    };
    (@[$prev: ty] &$ty: ty $(, $($tt:tt)*)?) => {
        $crate::meta_tuple_type!{@[$crate::Join<$prev, &$crate::MetaItem<$ty>>] $($($tt)*)?}
    };
    (@[$prev: ty] $ty: ty $(, $($tt:tt)*)?) => {
        $crate::meta_tuple_type!{@[$crate::Join<$prev, $crate::MetaItem<$ty>>] $($($tt)*)?}
    };
    (#$ty: ty $(, $($tt:tt)*)?) => {
        $crate::meta_tuple_type!{@[$ty] $($($tt)*)?}
    };
    (&mut $ty: ty $(, $($tt:tt)*)?) => {
        $crate::meta_tuple_type!{@[&mut $crate::MetaItem<$ty>] $($($tt)*)?}
    };
    (&$ty: ty $(, $($tt:tt)*)?) => {
        $crate::meta_tuple_type!{@[&$crate::MetaItem<$ty>] $($($tt)*)?}
    };
    ($ty: ty $(, $($tt:tt)*)?) => {
        $crate::meta_tuple_type!{@[$crate::MetaItem<$ty>] $($($tt)*)?}
    };
}

/// Add the `get` and `get_mut` function on subtraits of [`MetaAny`].
///
/// Syntax
///
/// ```
/// # use meta_tuple::*;
/// trait Metadata: MetaAny + 'static {}
///
/// impl_meta_box!(Metadata);
/// ```
#[macro_export]
macro_rules! impl_meta_box {
    ($trait: ident) => {
        const _: () = {
            use $crate::MetaAny;
            impl dyn $trait + '_ {
                /// Obtain an item if it exists in the [`MetaAny`].
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
                        $crate::ErasedInner::Struct(s) => {
                            let mut idx = 0;
                            while let Some(field) = s.get_field(idx) {
                                if let Some(result) = field.downcast_ref() {
                                    return Some(result);
                                }
                                idx += 1;
                            }
                            None
                        }
                    }
                }

                /// Obtain an item if it exists in the [`MetaAny`].
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
                        $crate::ErasedInnerMut::Struct(s) => {
                            let mut idx = 0;
                            loop {
                                // To get around a borrow checker issue with returned type.
                                //
                                // # Safety
                                //
                                // Safe since we can only return one field.
                                let s =
                                    unsafe { (s as *mut dyn $crate::MetaBundle).as_mut() }.unwrap();
                                let field = s.get_field_mut(idx)?;
                                if let Some(result) = field.downcast_mut() {
                                    return Some(result);
                                }
                                idx += 1;
                            }
                        }
                    }
                }
            }
        };
    };
}
