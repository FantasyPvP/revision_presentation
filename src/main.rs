use std::collections::HashMap;
use std::fs;

use rand::prelude::*;

use rocket::get;
use rocket::launch;
use rocket::routes;
use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket_dyn_templates::{context, Template};
use serde_json::{Result, Value};
use std::fs::File;
use std::io::Write;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Question {
    id: u32
    name: String,
    options: Vec<String>,
    answer: u32,
}

#[get("/")]
fn index() -> Template {
    Template::render("index", context! {})
}

#[get("/question")]
fn question() -> Template {
    let mut rng = rand::thread_rng();

    let filestring = fs::read_to_string("questions.json").unwrap();
    let questions: Vec<Question> =
        serde_json::from_str(&filestring).expect("Unable to parse JSON file");

    let question = questions[rng.gen_range(0..questions.len())].clone();
    println!("{:?}", question);

    Template::render("question", context! { question })
}

#[get("/writeq")]
fn writeq() -> &'static str {
    let filestring = fs::read_to_string("questions.json").unwrap();
    let mut questions: Vec<Question> =
        serde_json::from_str(&filestring).expect("Unable to parse JSON file");

    questions.push(Question {
        idx: questions.len(),
        name: String::from("test question 1"),
        options: vec!["answer 1".to_string(), "answer 2".to_string()],
        answer: 0,
    });
    let j = serde_json::to_string(&questions).unwrap();
    let mut out = File::create("questions.json").unwrap();
    write!(out, "{}", j);
    "ok"
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, question, writeq])
        .attach(Template::fairing())
}
