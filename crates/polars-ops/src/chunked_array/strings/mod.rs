#[cfg(feature = "strings")]
mod case;
#[cfg(feature = "strings")]
mod concat;
#[cfg(feature = "strings")]
mod escape_regex;
#[cfg(feature = "strings")]
mod extract;
#[cfg(feature = "find_many")]
mod find_many;
#[cfg(feature = "extract_jsonpath")]
mod json_path;
#[cfg(feature = "strings")]
mod namespace;
#[cfg(feature = "string_normalize")]
mod normalize;
#[cfg(feature = "string_pad")]
mod pad;
#[cfg(feature = "string_reverse")]
mod reverse;
#[cfg(feature = "strings")]
mod split;
#[cfg(feature = "strings")]
mod strip;
#[cfg(feature = "strings")]
mod substring;
#[cfg(all(not(feature = "nightly"), feature = "strings"))]
mod unicode_internals;

#[cfg(feature = "strings")]
pub use concat::*;
#[cfg(feature = "strings")]
pub use escape_regex::*;
#[cfg(feature = "find_many")]
pub use find_many::*;
#[cfg(feature = "extract_jsonpath")]
pub use json_path::*;
#[cfg(feature = "strings")]
pub use namespace::*;
#[cfg(feature = "string_normalize")]
pub use normalize::*;
use polars_core::prelude::*;
#[cfg(feature = "strings")]
pub use split::*;
#[cfg(feature = "strings")]
pub use strip::*;
#[cfg(feature = "strings")]
pub use substring::{substring_ternary_offsets_value, update_view};

pub trait AsString {
    fn as_string(&self) -> &StringChunked;
}

impl AsString for StringChunked {
    fn as_string(&self) -> &StringChunked {
        self
    }
}
