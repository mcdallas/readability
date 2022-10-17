extern crate html5ever;
extern crate markup5ever_rcdom;
extern crate regex;
extern crate url;
extern crate lazy_static;
#[cfg(feature = "reqwest")]
extern crate reqwest;

pub mod extractor;
pub mod scorer;
pub mod dom;
pub mod error;
