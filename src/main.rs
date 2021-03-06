#![allow(non_snake_case, non_camel_case_types)]
#![feature(plugin, decl_macro)]
#![plugin(rocket_codegen)]

extern crate rocket;

use std::io;
use rocket::response::NamedFile;

#[cfg(test)]
mod testsBin;

#[get("/")]
pub fn index() -> io::Result<NamedFile> {
    NamedFile::open("files/index.html")
}

fn main() {
    rocket::ignite().mount("/", routes![index]).launch();
}