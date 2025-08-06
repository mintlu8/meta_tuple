use crate::{MetaAny, MetaTuple};
use core::any::{type_name, TypeId};

pub trait MetaQuerySingle: MetaQuery {
    fn unique_type_id() -> TypeId;

    fn compatible<T: MetaQuerySingle>() -> bool {
        Self::unique_type_id() != T::unique_type_id()
    }
}

/// Query into a [`MetaTuple`].
/// 
/// By default implemented on tuples like `(&i32, &String)`.
/// 
/// # Safety
/// 
/// `validate` must ensure no duplicate fields are queried.
pub unsafe trait MetaQuery {
    type Output<'t>: Sized;
    type OutputPtr<'t>: Sized;
    fn query_ref<'t, T: MetaTuple + ?Sized + 't>(input: &'t T) -> Option<Self::Output<'t>>;

    fn query_mut<'t, T: MetaTuple + ?Sized + 't>(input: &'t mut T) -> Option<Self::Output<'t>> {
        if !Self::validate() {
            panic!("{} is not a valid MetaQuery.", type_name::<Self>());
        }
        let input = Self::query_mut_ptr(input)?;
        Some(unsafe { Self::from_ptr(input) })
    }

    fn query_dyn_ref<'t>(input: &'t dyn MetaAny) -> Option<Self::Output<'t>>;
    fn query_dyn_mut<'t>(input: &'t mut dyn MetaAny) -> Option<Self::Output<'t>> {
        if !Self::validate() {
            panic!("{} is not a valid MetaQuery.", type_name::<Self>());
        }
        let input = Self::query_dyn_mut_ptr(input)?;
        Some(unsafe { Self::from_ptr(input) })
    }

    fn query_mut_ptr<'t, T: MetaTuple + ?Sized + 't>(input: &'t T) -> Option<Self::OutputPtr<'t>>;
    fn query_dyn_mut_ptr<'t>(input: &'t dyn MetaAny) -> Option<Self::OutputPtr<'t>>;
    /// # Safety
    /// 
    /// Input must point to valid data.
    unsafe fn from_ptr<'t>(ptr: Self::OutputPtr<'t>) -> Self::Output<'t>;

    fn validate() -> bool;
}

unsafe impl<A: 'static> MetaQuery for &A {
    type Output<'t> = &'t A;

    fn query_ref<'t, T: MetaTuple + ?Sized + 't>(input: &'t T) -> Option<Self::Output<'t>> {
        input.get()
    }

    fn query_mut<'t, T: MetaTuple + ?Sized + 't>(input: &'t mut T) -> Option<Self::Output<'t>> {
        (input as &T).get()
    }

    fn query_dyn_ref<'t>(input: &'t dyn MetaAny) -> Option<Self::Output<'t>> {
        input.get()
    }

    fn query_dyn_mut<'t>(input: &'t mut dyn MetaAny) -> Option<Self::Output<'t>> {
        input.get()
    }

    type OutputPtr<'t> = &'t A;

    unsafe fn from_ptr<'t>(ptr: Self::OutputPtr<'t>) -> Self::Output<'t> {
        ptr
    }

    fn query_mut_ptr<'t, T: MetaTuple + ?Sized + 't>(input: &'t T) -> Option<Self::OutputPtr<'t>> {
        (input as &T).get()
    }

    fn query_dyn_mut_ptr<'t>(input: &'t dyn MetaAny) -> Option<Self::OutputPtr<'t>> {
        input.get()
    }

    fn validate() -> bool {
        true
    }
}

impl<A: 'static> MetaQuerySingle for &A {
    fn unique_type_id() -> TypeId {
        TypeId::of::<A>()
    }
}

unsafe impl<A: 'static> MetaQuery for &mut A {
    type Output<'t> = &'t mut A;

    fn query_ref<'t, T: MetaTuple + ?Sized + 't>(_: &'t T) -> Option<Self::Output<'t>> {
        None
    }

    fn query_mut<'t, T: MetaTuple + ?Sized + 't>(input: &'t mut T) -> Option<Self::Output<'t>> {
        input.get_mut()
    }

    fn query_dyn_ref<'t>(_: &'t dyn MetaAny) -> Option<Self::Output<'t>> {
        None
    }

    fn query_dyn_mut<'t>(input: &'t mut dyn MetaAny) -> Option<Self::Output<'t>> {
        input.get_mut()
    }

    type OutputPtr<'t> = *mut A;

    unsafe fn from_ptr<'t>(ptr: Self::OutputPtr<'t>) -> Self::Output<'t> {
        unsafe { ptr.as_mut().unwrap() }
    }

    fn query_mut_ptr<'t, T: MetaTuple + ?Sized + 't>(input: &'t T) -> Option<Self::OutputPtr<'t>> {
        input.get_mut_ptr()
    }

    fn query_dyn_mut_ptr<'t>(input: &'t dyn MetaAny) -> Option<Self::OutputPtr<'t>> {
        input.get_ptr()
    }

    fn validate() -> bool {
        true
    }
}

impl<A: 'static> MetaQuerySingle for &mut A {
    fn unique_type_id() -> TypeId {
        TypeId::of::<A>()
    }
}

unsafe impl<A: 'static> MetaQuery for Option<&A> {
    type Output<'t> = Option<&'t A>;

    fn query_ref<'t, T: MetaTuple + ?Sized + 't>(input: &'t T) -> Option<Self::Output<'t>> {
        Some(input.get())
    }

    fn query_mut<'t, T: MetaTuple + ?Sized + 't>(input: &'t mut T) -> Option<Self::Output<'t>> {
        Some((input as &T).get())
    }

    fn query_dyn_ref<'t>(input: &'t dyn MetaAny) -> Option<Self::Output<'t>> {
        Some(input.get())
    }

    fn query_dyn_mut<'t>(input: &'t mut dyn MetaAny) -> Option<Self::Output<'t>> {
        Some(input.get())
    }

    type OutputPtr<'t> = Option<&'t A>;

    unsafe fn from_ptr<'t>(ptr: Self::OutputPtr<'t>) -> Self::Output<'t> {
        ptr
    }

    fn query_mut_ptr<'t, T: MetaTuple + ?Sized + 't>(input: &'t T) -> Option<Self::OutputPtr<'t>> {
        Some((input as &T).get())
    }

    fn query_dyn_mut_ptr<'t>(input: &'t dyn MetaAny) -> Option<Self::OutputPtr<'t>> {
        Some(input.get())
    }

    fn validate() -> bool {
        true
    }
}

impl<A: 'static> MetaQuerySingle for Option<&A> {
    fn unique_type_id() -> TypeId {
        TypeId::of::<A>()
    }
}

unsafe impl<A: 'static> MetaQuery for Option<&mut A> {
    type Output<'t> = Option<&'t mut A>;

    fn query_ref<'t, T: MetaTuple + ?Sized + 't>(_: &'t T) -> Option<Self::Output<'t>> {
        Some(None)
    }

    fn query_mut<'t, T: MetaTuple + ?Sized + 't>(input: &'t mut T) -> Option<Self::Output<'t>> {
        Some(input.get_mut())
    }

    fn query_dyn_ref<'t>(_: &'t dyn MetaAny) -> Option<Self::Output<'t>> {
        Some(None)
    }

    fn query_dyn_mut<'t>(input: &'t mut dyn MetaAny) -> Option<Self::Output<'t>> {
        Some(input.get_mut())
    }

    type OutputPtr<'t> = Option<*mut A>;

    unsafe fn from_ptr<'t>(ptr: Self::OutputPtr<'t>) -> Self::Output<'t> {
        ptr.map(|ptr| unsafe { ptr.as_mut().unwrap() })
    }

    fn query_mut_ptr<'t, T: MetaTuple + ?Sized + 't>(input: &'t T) -> Option<Self::OutputPtr<'t>> {
        Some(input.get_mut_ptr())
    }

    fn query_dyn_mut_ptr<'t>(input: &'t dyn MetaAny) -> Option<Self::OutputPtr<'t>> {
        Some(input.get_ptr())
    }

    fn validate() -> bool {
        true
    }
}

impl<A: 'static> MetaQuerySingle for Option<&mut A> {
    fn unique_type_id() -> TypeId {
        TypeId::of::<A>()
    }
}

macro_rules! validate {
    () => { true };
    ($A: ident $($T: ident)*) => {
        $($A::compatible::<$T>() && )* true && validate!($($T)*)
    };
}

macro_rules! impl_meta_query {
    ($($T: ident)*) => {
        #[allow(unused_variables, non_snake_case, clippy::unused_unit)]
        unsafe impl<$($T: MetaQuerySingle + 'static),*> MetaQuery for ($($T,)*) {
            type Output<'t> = ($($T::Output<'t>,)*);

            fn query_ref<'t, T: MetaTuple + ?Sized + 't>(input: &'t T) -> Option<Self::Output<'t>> {
                Some(($($T::query_ref(input)?,)*))
            }

            fn query_dyn_ref<'t>(input: &'t dyn MetaAny) -> Option<Self::Output<'t>> {
                Some(($($T::query_dyn_ref(input)?,)*))
            }

            type OutputPtr<'t> = ($($T::OutputPtr<'t>,)*);

            unsafe fn from_ptr<'t>(ptr: Self::OutputPtr<'t>) -> Self::Output<'t> {
                let ($($T,)*) = ptr;
                ($(unsafe {$T::from_ptr($T)},)*)
            }

            fn query_mut_ptr<'t, T: MetaTuple + ?Sized + 't>(input: &'t T) -> Option<Self::OutputPtr<'t>> {
                Some(($($T::query_mut_ptr(input)?,)*))
            }

            fn query_dyn_mut_ptr<'t>(input: &'t dyn MetaAny) -> Option<Self::OutputPtr<'t>> {
                Some(($($T::query_dyn_mut_ptr(input)?,)*))
            }

            fn validate() -> bool {
                validate!($($T)*)
            }
        }
    };
}

impl_meta_query!();
impl_meta_query!(T0);
impl_meta_query!(T0 T1);
impl_meta_query!(T0 T1 T2);
impl_meta_query!(T0 T1 T2 T3);
impl_meta_query!(T0 T1 T2 T3 T4);
impl_meta_query!(T0 T1 T2 T3 T4 T5);
impl_meta_query!(T0 T1 T2 T3 T4 T5 T6);
impl_meta_query!(T0 T1 T2 T3 T4 T5 T6 T7);
impl_meta_query!(T0 T1 T2 T3 T4 T5 T6 T7 T8);
impl_meta_query!(T0 T1 T2 T3 T4 T5 T6 T7 T8 T9);
impl_meta_query!(T0 T1 T2 T3 T4 T5 T6 T7 T8 T9 T10);
impl_meta_query!(T0 T1 T2 T3 T4 T5 T6 T7 T8 T9 T10 T11);
