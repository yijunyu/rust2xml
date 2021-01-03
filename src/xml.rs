//! A module to provide functions for XML <-> Rust serialize and deserialize.
//!
//! *This module is available if syn-serde is built with the `"xml"` feature.*

use syn_serde::Syn;

// Serialize [`Syn`] type into XML data.

/// Serialize the given [`Syn`] type as a String of XML.
///
/// This function is equivalent to the following code:
///
/// ```rust
/// # fn to_string(syn_file: &syn::File) -> String {
/// use syn_serde::Syn;
///
/// let serializable_file = syn_file.to_adapter();
/// serde_xml_rs::to_string(&serializable_file).unwrap()
/// # }
/// ```
#[inline]
pub fn to_string<S>(syn: &S) -> String
where
    S: Syn,
{
    let adapter = syn.to_adapter();
    // serde_xml_rs::ser::to_string(&adapter).expect("no error")
    quick_xml::se::to_string(&adapter).expect("no error")
}
