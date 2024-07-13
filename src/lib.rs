#![no_std] #![feature(negative_impls, auto_traits, const_type_id)]
#![allow(suspicious_auto_trait_impls)]

extern crate alloc;

pub(crate) mod anything;
pub(crate) mod nothing;

pub use anything::*;
pub use nothing::*;

pub type Result<T> = core::result::Result<T, Anything>;
pub type Exception<T> = core::result::Result<T, Anything>;