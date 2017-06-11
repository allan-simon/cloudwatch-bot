//! TODO

#![feature(plugin, custom_derive)]
#![plugin(rocket_codegen)]
#![cfg_attr(feature="clippy", plugin(clippy))]

#![deny(
  warnings,
  unused,bad_style,
  missing_docs,
  missing_copy_implementations,
  missing_debug_implementations,
  trivial_casts,
  trivial_numeric_casts,
  unused_extern_crates,
  unused_import_braces,
  unused_qualifications
)]

extern crate hyper;
extern crate rocket;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate slack_api;

mod errors;
mod http;
mod notify;
mod sns;

fn main() {
  rocket::ignite()
    .mount("/", routes![http::notify_route, http::health_check_route])
    .catch(errors![http::not_found])
    .launch();
}
