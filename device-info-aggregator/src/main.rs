#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

mod web;
mod info;
mod domain;

fn main() {
    let routes = routes![web::device_crud::register_new_device, web::device_crud::delete_device, 
                web::device_info::bat, web::device_info::time, web::device_info::update_battery_report];

    rocket::ignite().mount("/", routes).launch();
}