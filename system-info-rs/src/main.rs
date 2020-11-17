#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

mod web;
mod info;

fn main() {
    let routes = routes![web::web_controller::bat, web::web_controller::time];

    rocket::ignite().mount("/", routes).launch();
}