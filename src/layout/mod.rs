//! The [common layout tables][1].
//!
//! [1]: https://www.microsoft.com/typography/otspec/chapter2.htm

mod class;
mod correction;
mod coverage;
mod directory;

pub mod feature;
pub mod lookup;
pub mod script;

pub use self::class::{Class, Class1, Class2, ClassRange};
pub use self::correction::{Correction, Device, Variation};
pub use self::coverage::{Coverage, Coverage1, Coverage2, CoverageRange};
pub use self::directory::Directory;
pub use self::feature::Features;
pub use self::lookup::Lookups;
pub use self::script::Scripts;
