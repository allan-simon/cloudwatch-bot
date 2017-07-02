//! cloudwatch-bot is a simple AWS SNS (Simple Notification Service) listener
//! that posts alarms updates to various chat systems.

#![feature(plugin, custom_derive)]
#![plugin(rocket_codegen)]

#![cfg_attr(feature="clippy", plugin(clippy))]
#![cfg_attr(feature="clippy", deny(clippy, clippy_pedantic))]
#![cfg_attr(feature="clippy", allow(missing_docs_in_private_items))]

// Clippy false positives
#![cfg_attr(feature="clippy", allow(needless_pass_by_value))]
#![cfg_attr(feature="clippy", allow(used_underscore_binding))]

#![deny(
  warnings, bad_style, unused, future_incompatible,
  trivial_casts, trivial_numeric_casts,
  missing_docs, missing_copy_implementations, missing_debug_implementations,
  unused, unused_extern_crates, unused_import_braces, unused_qualifications
)]

///////////////////
// Extern crates //
///////////////////

extern crate futures;
extern crate hyper;
extern crate hyper_tls;
#[macro_use]
extern crate lazy_static;
extern crate rocket;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate tokio_core;

/////////////
// Modules //
/////////////

mod http;
mod model;
mod services;

//////////
// Main //
//////////

fn main() {
    http::setup_server().launch();
}
