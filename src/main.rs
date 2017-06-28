//! cloudwatch-bot is a simple AWS SNS (Simple Notification Service) listener
//! that posts alarms updates to various chat systems.

#![feature(plugin, custom_derive)]

#![cfg_attr(feature="clippy", plugin(clippy))]
#![cfg_attr(feature="clippy", deny(clippy, clippy_pedantic))]
#![cfg_attr(feature="clippy", allow(missing_docs_in_private_items))]

#![deny(
  warnings, bad_style, unused, future_incompatible,
  trivial_casts, trivial_numeric_casts,
  missing_docs, missing_copy_implementations, missing_debug_implementations,
  unused, unused_extern_crates, unused_import_braces, unused_qualifications
)]

///////////////////
// Extern crates //
///////////////////

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

/////////////
// Modules //
/////////////

mod sns;

//////////
// Main //
//////////

fn main() {}
