#[cfg(any(feature = "derive", feature = "typed-uuid"))]
pub use ::diesel;
#[cfg(feature = "derive")]
pub use ::indexmap;
#[cfg(feature = "serde")]
pub use ::serde;
pub use ::std;
#[cfg(feature = "typed-uuid")]
pub use ::uuid;

#[cfg(all(feature = "derive", feature = "rustc-hash"))]
type Hasher = rustc_hash::FxBuildHasher;

#[cfg(all(feature = "derive", not(feature = "rustc-hash")))]
type Hasher = std::hash::RandomState;

#[cfg(feature = "derive")]
pub type IndexMap<K, V> = indexmap::IndexMap<K, V, Hasher>;

#[cfg(feature = "derive")]
#[must_use]
pub fn new_indexmap<K, V>() -> IndexMap<K, V> {
    IndexMap::with_hasher(Hasher::default())
}

pub mod deep_clone {
    /// Helper wrapper to avoid conflicting implementations
    pub struct Wrap<T>(pub T);

    pub trait DeepClonePlain {
        type Output;

        fn deep_clone(&self) -> Self::Output;
    }

    pub trait DeepCloneRef {
        type Output;

        fn deep_clone(&self) -> Self::Output;
    }

    pub trait DeepCloneTuple {
        type Output;

        fn deep_clone(&self) -> Self::Output;
    }

    impl<T> DeepClonePlain for Wrap<T>
    where
        T: Clone,
    {
        type Output = T;

        fn deep_clone(&self) -> Self::Output {
            self.0.clone()
        }
    }

    impl<T> DeepCloneRef for &Wrap<&T>
    where
        T: Clone,
    {
        type Output = T;

        fn deep_clone(&self) -> Self::Output {
            self.0.clone()
        }
    }

    #[rustfmt::skip]
    mod impls {
        macro_rules! impl_deep_clone_for_tuples {
            ($(($T:ident, $idx:tt)),+) => {
                impl<$($T: Clone),+> $crate::__private::deep_clone::DeepCloneTuple for &&$crate::__private::deep_clone::Wrap<($(&$T),+,)> {
                    type Output = ($($T),+,);

                    fn deep_clone(&self) -> Self::Output {
                        ($(self.0.$idx.clone()),+,)
                    }
                }
            };
        }

        impl_deep_clone_for_tuples!((T1, 0));
        impl_deep_clone_for_tuples!((T1, 0), (T2, 1));
        impl_deep_clone_for_tuples!((T1, 0), (T2, 1), (T3, 2));
        impl_deep_clone_for_tuples!((T1, 0), (T2, 1), (T3, 2), (T4, 3));
        impl_deep_clone_for_tuples!((T1, 0), (T2, 1), (T3, 2), (T4, 3), (T5, 4));
        impl_deep_clone_for_tuples!((T1, 0), (T2, 1), (T3, 2), (T4, 3), (T5, 4), (T6, 5));
        impl_deep_clone_for_tuples!((T1, 0), (T2, 1), (T3, 2), (T4, 3), (T5, 4), (T6, 5), (T7, 6));
        impl_deep_clone_for_tuples!((T1, 0), (T2, 1), (T3, 2), (T4, 3), (T5, 4), (T6, 5), (T7, 6), (T8, 7));
        impl_deep_clone_for_tuples!((T1, 0), (T2, 1), (T3, 2), (T4, 3), (T5, 4), (T6, 5), (T7, 6), (T8, 7), (T9, 8));
        impl_deep_clone_for_tuples!((T1, 0), (T2, 1), (T3, 2), (T4, 3), (T5, 4), (T6, 5), (T7, 6), (T8, 7), (T9, 8), (T10, 9));
        impl_deep_clone_for_tuples!((T1, 0), (T2, 1), (T3, 2), (T4, 3), (T5, 4), (T6, 5), (T7, 6), (T8, 7), (T9, 8), (T10, 9), (T11, 10));
        impl_deep_clone_for_tuples!((T1, 0), (T2, 1), (T3, 2), (T4, 3), (T5, 4), (T6, 5), (T7, 6), (T8, 7), (T9, 8), (T10, 9), (T11, 10), (T12, 11));
        impl_deep_clone_for_tuples!((T1, 0), (T2, 1), (T3, 2), (T4, 3), (T5, 4), (T6, 5), (T7, 6), (T8, 7), (T9, 8), (T10, 9), (T11, 10), (T12, 11), (T13, 12));
        impl_deep_clone_for_tuples!((T1, 0), (T2, 1), (T3, 2), (T4, 3), (T5, 4), (T6, 5), (T7, 6), (T8, 7), (T9, 8), (T10, 9), (T11, 10), (T12, 11), (T13, 12), (T14, 13));
        impl_deep_clone_for_tuples!((T1, 0), (T2, 1), (T3, 2), (T4, 3), (T5, 4), (T6, 5), (T7, 6), (T8, 7), (T9, 8), (T10, 9), (T11, 10), (T12, 11), (T13, 12), (T14, 13), (T15, 14));
        impl_deep_clone_for_tuples!((T1, 0), (T2, 1), (T3, 2), (T4, 3), (T5, 4), (T6, 5), (T7, 6), (T8, 7), (T9, 8), (T10, 9), (T11, 10), (T12, 11), (T13, 12), (T14, 13), (T15, 14), (T16, 15));
    }

    #[cfg(test)]
    mod tests {
        use super::{DeepClonePlain, DeepCloneRef, DeepCloneTuple, Wrap};

        #[test]
        #[expect(
            clippy::needless_borrow,
            reason = "we are testing that autoderef specialization is working"
        )]
        fn deep_clone_impl() {
            let _: u32 = (&&Wrap(&0u32)).deep_clone();
            let _: u32 = (&&Wrap(0u32)).deep_clone();
            let _: (u32,) = (&&Wrap((&0u32,))).deep_clone();
            let _: (u32, u32) = (&&Wrap((&0u32, &0u32))).deep_clone();
        }
    }
}
