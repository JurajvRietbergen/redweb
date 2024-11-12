use rocket::*;
use rocket::fs::{relative, FileServer};
use rocket_dyn_templates::Template;

#[get("/")]
fn index() -> Template {
    Template::render("index", &())
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/static", FileServer::from(relative!("static")))
        .mount("/", routes![index])
        .attach(Template::fairing())
}
