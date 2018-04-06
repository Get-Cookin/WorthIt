#![allow(non_snake_case, non_camel_case_types)]
#![feature(plugin, decl_macro)]
#![plugin(rocket_codegen)]

extern crate rocket;

use rocket::response::Content;
use rocket::http::ContentType;

#[cfg(test)]
mod tests;


#[get("/")]
pub fn index<'a>() -> rocket::response::Content<&'a str> {
    Content(ContentType::HTML, include_str!("files/index.html"))
}

fn main() {
    rocket::ignite().mount("/", routes![index]).launch();
}