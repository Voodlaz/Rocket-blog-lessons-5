use crate::posts::forms::{NewPostForm};

use rocket_contrib::templates::Template;
use std::collections::HashMap;

use rocket::request::{Form, FlashMessage};
use rocket::response::{Flash, Redirect};

#[get("/new_post")]
pub fn new_post(signal: Option<FlashMessage<'_, '_>>) -> Template {
    let mut context: HashMap<&str, &str> = HashMap::new();
    if let Some(signal) = signal {
        let signal_error = signal.msg();
        context.insert("validation-error".into(), signal_error.into());
    }
    Template::render("new_post", context)
}

#[post("/new_post", data="<form>")]
pub fn new_post_form(form: Form<NewPostForm>) -> Flash<Redirect> {
    let form = form.into_inner();

    if form.name == "" {
        Flash::error(Redirect::to("/new_post"), "no-name");}

    if form.body == "" {
        Flash::error(Redirect::to("/new_post"), "no-body");}

    if form.name == "" && form.body == "" {
        Flash::error(Redirect::to("/new_post"), "no-body-name");}

    if form.name != "" && form.body != "" {
        Flash::success(Redirect::to("/new_post"), "all working")
    } else {
        Flash::error(Redirect::to("/new_post"), "ololo")
    }
}
