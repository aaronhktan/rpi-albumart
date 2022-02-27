#[macro_use]
extern crate rocket;

use rocket::fs::{relative, FileServer};
use rocket_dyn_templates::Template;

mod tera;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![tera::hello])
        .register("/", catchers![tera::not_found])
        .attach(Template::fairing())
        .mount("/static", FileServer::from(relative!("static")))
}
