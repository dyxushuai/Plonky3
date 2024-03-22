//! A framework for operating over the unit circle of a finite field,
//! following the [Circle STARKs paper](https://eprint.iacr.org/2024/278) by Haböck, Levit and Papini.

#![cfg_attr(all(not(test), not(feature = "multi_thread")), no_std)]

extern crate alloc;

mod domain;
mod pcs;
mod twiddles;
mod util;

pub use pcs::*;

#[allow(dead_code)]
#[cfg(feature = "multi_thread")]
pub mod multi_thread {
    use ::std::sync::{Arc, RwLock};

    include!("cfft.rs");
}

#[allow(dead_code)]
#[cfg(not(feature = "multi_thread"))]
pub mod single_thread {
    mod seal {
        use ::core::cell::{Ref, RefCell, RefMut};

        #[derive(Default, Debug)]
        pub struct RwLock<T>(RefCell<T>);

        // Mimic `RwLock`'s API
        impl<T> RwLock<T> {
            pub fn new(value: T) -> Self {
                Self(RefCell::new(value))
            }

            #[inline]
            pub fn read(&self) -> Result<Ref<'_, T>, ::core::convert::Infallible> {
                Ok(self.0.borrow())
            }

            #[inline]
            pub fn write(&self) -> Result<RefMut<'_, T>, ::core::convert::Infallible> {
                Ok(self.0.borrow_mut())
            }
        }
    }
    use ::alloc::rc::Rc as Arc;
    use seal::RwLock;

    include!("cfft.rs");
}

#[cfg(feature = "multi_thread")]
pub use multi_thread::*;
#[cfg(not(feature = "multi_thread"))]
pub use single_thread::*;
