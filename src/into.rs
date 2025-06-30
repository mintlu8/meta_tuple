use crate::{meta_tuple, MetaTuple};


/// A trait that converts a more user friendly concrete type into a [`MetaTuple`].
/// 
/// By default this is implemented on static tuples.
pub trait IntoMetaTuple {
    fn into_meta_tuple(self) -> impl MetaTuple;
}

impl<T> IntoMetaTuple for T where T: MetaTuple {
    fn into_meta_tuple(self) -> impl MetaTuple {
        self
    }
}

macro_rules! impl_tuple {
    ($($T: ident),*) => {
        #[allow(non_snake_case)]
        impl<$($T: 'static,)*> IntoMetaTuple for ($($T,)*) {
            fn into_meta_tuple(self) -> impl MetaTuple {
                let ($($T,)*) = self;
                meta_tuple!($($T,)*)
            }
        }
    };
}

impl_tuple!(A);
impl_tuple!(A, B);
impl_tuple!(A, B, C);
impl_tuple!(A, B, C, D);
impl_tuple!(A, B, C, D, E);
impl_tuple!(A, B, C, D, E, F);
impl_tuple!(A, B, C, D, E, F, G);
impl_tuple!(A, B, C, D, E, F, G, H);
impl_tuple!(A, B, C, D, E, F, G, H, I);
impl_tuple!(A, B, C, D, E, F, G, H, I, J);
impl_tuple!(A, B, C, D, E, F, G, H, I, J, K);
impl_tuple!(A, B, C, D, E, F, G, H, I, J, K, L);