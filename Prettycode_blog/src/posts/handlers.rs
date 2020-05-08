use crate::posts::forms::{NewPostForm};

use rocket_contrib::templates::Template;
use std::collections::HashMap;

use rocket::request::{Form, FlashMessage};
use rocket::response::{Flash, Redirect};

#[get("/new_post")]
pub fn new_post() -> Template {
    let context: HashMap<&str, &str> = HashMap::new();
    Template::render("new_post", &context)
}

#[post("/new_post", data="<form>")]
pub fn new_post_form(form: Form<NewPostForm>) -> Flash<Redirect> {todo!}
