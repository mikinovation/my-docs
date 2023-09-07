mod core;
mod api;

use axum::{
    routing::{get},
    Router,
};
use serde::{Deserialize, Serialize};
use std::env;
use std::net::SocketAddr;
use std::sync::Arc;
use axum::extract::Extension;
use hyper::header::CONTENT_TYPE;
use tower_http::cors::{Any, CorsLayer, Origin};
use api::handler::article::article_handler::{all_article};
use core::infrastructure::repository::article::article_repository::ArticleRepository;
use core::infrastructure::repository::article::article_repository::ArticleRepositoryForDb;
use sqlx::postgres::PgPool;
use dotenv::dotenv;

#[tokio::main]
async fn main() {
    // logging
    let log_level = env::var("RUST_LOG").unwrap_or("info".to_string());
    env::set_var("RUST_LOG", log_level);
    tracing_subscriber::fmt::init();
    dotenv().ok();

    let database_url = &env::var("DATABASE_URL").expect("undefined [DATABASE_URL]");
    tracing::debug!("start connect database...");
    let pool = PgPool::connect(database_url)
        .await
        .expect(&format!("fail connect database, url is [{}]", database_url));

    let app = create_app(
        ArticleRepositoryForDb::new(pool.clone()),
    );

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    tracing::debug!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

fn create_app<Article: ArticleRepository>(
    article_repository: Article,
) -> Router {
    Router::new()
        .route("/", get(root))
        .route("/articles", get(all_article::<Article>))
/*        .route(
            "/todos/:id",
            get(find_todo::<Todo>)
                // .delete(delete_todo::<Todo>)
                // .patch(update_todo::<Todo>),
        )
*/
        .layer(Extension(Arc::new(article_repository)))
        .layer(
            CorsLayer::new()
                .allow_origin(Origin::exact("http://localhost:3001".parse().unwrap()))
                .allow_methods(Any)
                .allow_headers(vec![CONTENT_TYPE]),
        )
}

async fn root() -> &'static str {
    "Hello, World!"
}

#[derive(Deserialize)]
struct CreateUser {
    username: String,
}

#[derive(Serialize)]
struct User {
    id: u64,
    username: String,
}
