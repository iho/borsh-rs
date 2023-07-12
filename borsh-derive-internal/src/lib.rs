#![recursion_limit = "128"]

mod attribute_helpers;
mod enum_de;
mod enum_discriminant_map;
mod enum_ser;
mod struct_de;
mod struct_ser;
mod union_de;
mod union_ser;

pub use enum_de::enum_de;
pub use enum_ser::enum_ser;
pub use struct_de::struct_de;
pub use struct_ser::struct_ser;
pub use union_de::union_de;
pub use union_ser::union_ser;

#[cfg(test)]
pub mod test_helpers;
pub use attribute_helpers::check_use_discriminant;
