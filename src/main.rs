use sqlx::PgPool;
use axum::{
    routing::{delete, get},
    Extension, Router,
};

use tokio::sync::broadcast::{channel, Sender};
use tracing::info;

mod errors;
mod models;
mod todo_controller;
mod templates;

pub type TodosStream = Sender<models::TodoUpdate>;

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
}

#[shuttle_runtime::main]
async fn main(#[shuttle_shared_db::Postgres] db: PgPool) -> shuttle_axum::ShuttleAxum {
    sqlx::migrate!()
        .run(&db)
        .await
        .expect("Looks like something went wrong with migrations :(");

    let router = init_router(db);

    Ok(router.into())
}

pub fn init_router(db: PgPool) -> Router {
    let (tx, _rx) = channel::<models::TodoUpdate>(10);
    let state = AppState { db };

    let mut router = Router::new()
        .route("/", get(todo_controller::home))
        .route("/stream", get(todo_controller::stream))
        .route("/styles.css", get(todo_controller::styles))
        .route("/todos", get(todo_controller::fetch_todos).post(todo_controller::create_todo))
        .route("/todos/:id", delete(todo_controller::delete_todo))
        .route("/todos/stream", get(todo_controller::handle_stream))
        .with_state(state)
        .layer(Extension(tx));

    // Causes the front end to break if constantly adding and deleting... 
    // if cfg!(debug_assertions) {
    //     router = router.layer(tower_livereload::LiveReloadLayer::new());
    // }

    router
}