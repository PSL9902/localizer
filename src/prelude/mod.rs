#[cfg(not(feature = "std"))]
//#[cfg(feature = "no_std")]
pub mod no_std_prelude;
#[cfg(not(feature = "std"))]
//#[cfg(feature = "no_std")]
pub use no_std_prelude::*;

#[cfg(feature = "std")]
pub mod std_prelude;
#[cfg(feature = "std")]
pub use std_prelude::*;

pub mod funcs;
pub use funcs::*;
