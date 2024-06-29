#![cfg_attr(not(test), no_std)]

macro_rules! align_impl {
    ($name:ident, $align:literal) => {
        /// A wrapper type that ensures that the inner type is aligned **at
        /// least** to the specified boundary.
        #[repr(align($align))]
        pub struct $name<T>(T);

        impl<T> $name<T> {
            #[must_use]
            pub const fn new(inner: T) -> Self {
                Self(inner)
            }
        }

        impl<T> core::ops::Deref for $name<T> {
            type Target = T;
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl<T> core::ops::DerefMut for $name<T> {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }
    };
}

align_impl!(Align2, 2);
align_impl!(Align4, 4);
align_impl!(Align8, 8);
align_impl!(Align16, 16);
align_impl!(Align32, 32);
align_impl!(Align64, 64);
align_impl!(Align128, 128);
align_impl!(Align256, 256);
align_impl!(Align512, 512);
align_impl!(Align1024, 1024);
align_impl!(Align2048, 2048);
align_impl!(Align4096, 4096);
