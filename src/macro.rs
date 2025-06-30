#[allow(unused)]
use crate::{MetaTuple, MetaItem};

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
        meta_tuple!(@[($prev, $e)] $($($rest)*)?)
    };
    (@[$prev: expr] &mut $e: expr $(, $($rest: tt)*)?) => {
        meta_tuple!(@[($prev, $crate::MetaItem::from_mut(&mut $e))] $($($rest)*)?)
    };
    (@[$prev: expr] &$e: expr $(, $($rest: tt)*)?) => {
        meta_tuple!(@[($prev, $crate::MetaItem::from_ref(&$e))] $($($rest)*)?)
    };
    (@[$prev: expr] $e: expr $(, $($rest: tt)*)?) => {
        meta_tuple!(@[($prev, $crate::MetaItem($e))] $($($rest)*)?)
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

/// Implement [`MetaTuple`] for a type, making it equivalent to [`MetaItem<T>`].
/// 
/// This is useful to satisfy API constraints.
/// 
/// # Syntax
/// 
/// This is equivalent to a derive macro.
/// 
/// ```
/// // Equivalent to impl MetaTuple for MyType {}
/// impl_meta_tuple!(MyType)
/// 
/// // Equivalent to impl<T: Copy> MetaTuple for MyType <T> {}
/// impl_meta_tuple!([T: Copy]MyType[T])
/// ```
#[macro_export]
macro_rules! impl_meta_tuple {
    ($ty: ident) => {
        impl $crate::MetaBox for $ty {
            fn as_erased(&self) -> $crate::ErasedInner<'_> {
                $crate::ErasedInner::Any(self)
            }
            fn as_erased_mut(&mut self) -> $crate::ErasedInnerMut<'_> {
                $crate::ErasedInnerMut::Any(self)
            }
        }
        impl $crate::MetaTuple for $ty {
            fn get<__T: 'static>(&self) -> Option<&__T> {
                (self as &dyn $crate::Any).downcast_ref()
            }
            fn get_mut<__T: 'static>(&mut self) -> Option<&mut __T> {
                (self as &mut dyn $crate::Any).downcast_mut()
            }
        }
    };

    ($ty: ident [$($a: tt)*]) => {
        $crate::impl_meta_tuple!{[$($a)*] $ty [$($a)*]}
    };

    ([$($a: tt)*]$ty: ident [$($b: tt)*]) => {
        impl<$($a)*> $crate::MetaBox for $ty<$($b)*> where Self: 'static {
            fn as_erased(&self) -> $crate::ErasedInner<'_> {
                $crate::ErasedInner::Any(self)
            }
            fn as_erased_mut(&mut self) -> $crate::ErasedInnerMut<'_> {
                $crate::ErasedInnerMut::Any(self)
            }
        }
        impl<$($a)*> $crate::MetaTuple for $ty<$($b)*> where Self: 'static {
            /// Obtain an item, if exists.
            fn get<__T: 'static>(&self) -> Option<&__T> {
                (self as &dyn $crate::Any).downcast_ref()
            }
            /// Obtain a mutable item, if exists.
            fn get_mut<__T: 'static>(&mut self) -> Option<&mut __T> {
                (self as &mut dyn $crate::Any).downcast_mut()
            }
        }
    };
}
