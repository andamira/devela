#[macro_export]
#[allow(missing_docs)]
#[cfg_attr(cargo_primary_package, doc(hidden))]
macro_rules! items· { ( $($item:item)* ) => { $($item)* }; }
#[doc(inline)]
pub use items· as items;
