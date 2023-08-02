use std::{
    any::TypeId,
    ptr::{addr_of, NonNull},
};

use crate::store::ItemType;

pub trait Bundle {
    fn type_id() -> TypeId;
    fn for_each_type(f: impl FnMut(&ItemType));
    fn get_components(self, f: impl FnMut(NonNull<u8>, &ItemType));
}

macro_rules! tuple_impl {
    ($($name:ident),*) => {
        #[allow(non_snake_case)]
        impl<$($name: 'static + Send + Sync),*> Bundle for ($($name,)*) {
            fn type_id() -> TypeId {
                TypeId::of::<($($name,)*)>()
            }

            fn for_each_type(mut f: impl FnMut(&ItemType)) {
                $(f(&ItemType::of::<$name>());)*
            }

            fn get_components(self, mut f: impl FnMut(NonNull<u8>, &ItemType)) {
                let ($($name,)+) = self;
                $(f(unsafe { NonNull::new_unchecked(addr_of!($name) as *mut u8) }, &ItemType::of::<$name>());)*
            }
        }
    };
}

tuple_impl!(A);
tuple_impl!(A, B);
tuple_impl!(A, B, C);
tuple_impl!(A, B, C, D);
tuple_impl!(A, B, C, D, E);
tuple_impl!(A, B, C, D, E, F);
tuple_impl!(A, B, C, D, E, F, G);
tuple_impl!(A, B, C, D, E, F, G, H);
tuple_impl!(A, B, C, D, E, F, G, H, I);
tuple_impl!(A, B, C, D, E, F, G, H, I, J);
tuple_impl!(A, B, C, D, E, F, G, H, I, J, K);
tuple_impl!(A, B, C, D, E, F, G, H, I, J, K, L);
tuple_impl!(A, B, C, D, E, F, G, H, I, J, K, L, M);
tuple_impl!(A, B, C, D, E, F, G, H, I, J, K, L, M, N);
tuple_impl!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O);
tuple_impl!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P);
tuple_impl!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q);
tuple_impl!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R);
tuple_impl!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S);
tuple_impl!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T);
tuple_impl!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U);
tuple_impl!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V);
tuple_impl!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W);
tuple_impl!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X);
tuple_impl!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y);
tuple_impl!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z);
