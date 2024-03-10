use crate::models;
use askama::Template;

#[derive(Template)]
#[template(path = "index.html.j2")]
pub struct HelloTemplate;

#[derive(Template)]
#[template(path = "stream.html.j2")]
pub struct StreamTemplate;

#[derive(Template)]
#[template(path = "todos.html.j2")]
pub struct Records {
    pub todos: Vec<models::Todo>,
}

#[derive(Template)]
#[template(path = "todo.html.j2")]
pub struct TodoNewTemplate {
    pub todo: models::Todo,
}