pub mod database;
pub mod handlers;

use anyhow::Result;
use axum::{response::Html, routing::get, Extension, Router};
use sqlx::SqlitePool;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<()> {
    // Load enviroment variables from .env if available
    dotenv::dotenv().ok();

    // Initialize the database and obtain a connection pool
    let connection_pool = init_db().await?;

    // Initialize the Axum routing service
    let app = router(connection_pool);

    let listener = TcpListener::bind("127.0.0.1:3001").await?;

    // Start the server
    println!("->> LISTENING on http://{}\n", listener.local_addr()?);
    println!("### Endpoints");
    println!("->> http://localhost:3001/             => Home page");
    println!("->> http://localhost:3001/employees    => All employees");
    println!("->> http://localhost:3001/projects     => All projects");
    println!("->> http://localhost:3001/tasks        => All tasks\n");

    // Run the server
    axum::
        serve(listener, app.into_make_service())
        .await
        .unwrap();
    Ok(())
}

// Create a database connection pool. Run any migrations.
pub async fn init_db() -> Result<SqlitePool> {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let connection_pool = SqlitePool::connect(&database_url).await?;
    sqlx::migrate!().run(&connection_pool).await?;
    Ok(connection_pool)
}

// Build the overall web service router.
// Constructing the router in a function makes it easy to re-use in unit tests.
fn router(connection_pool: SqlitePool) -> Router {
    Router::new()
        // Add the web view
        .nest_service("/", view_service())
        // Add the REST APIs
        .nest_service("/employees", handlers::employees::employees_api())
        .nest_service("/projects", handlers::projects::projects_api())
        .nest_service("/tasks", handlers::tasks::assigned_api())
        // Add the connection pool as a "layer", available for dependency injection
        .layer(Extension(connection_pool))
}

pub fn view_service() -> Router {
    Router::new().route("/", get(index_page))
}

const INDEX_PAGE: &str = include_str!("views/index.html");

async fn index_page() -> Html<&'static str> {
    Html(INDEX_PAGE)
}
