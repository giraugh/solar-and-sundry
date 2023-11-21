mod data;

use std::{env, net::SocketAddr, sync::Arc};

use axum::{
    extract::State,
    headers::authorization::{Authorization, Bearer},
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
    routing::{delete, get, post, put},
    Router, TypedHeader,
};

use tokio::sync::Mutex;
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

pub type DbClient = Arc<Mutex<libsql_client::Client>>;

#[derive(Clone, Debug)]
struct ApiState {
    pub api_token: String,
    pub db: DbClient,
}

#[tokio::main]
async fn main() {
    // Init logging
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env())
        .init();

    // TODO: Define cors layer

    // Get db client
    let db_url = std::env::var("DB_URL").unwrap_or("file:///tmp/sas.db".into());
    let db = libsql_client::Client::from_config(libsql_client::Config {
        url: url::Url::parse(&db_url).unwrap(),
        auth_token: None,
    })
    .await
    .unwrap();
    let db = Arc::new(Mutex::new(db));

    // Create db tables if they don't exist
    use data::page::Page;
    create_tables![db; Page];

    // Init api state
    let api_state = ApiState {
        api_token: env::var("API_TOKEN").expect("Expected API_TOKEN environment variable"),
        db,
    };

    // Define private routes
    let private_routes = Router::new()
        .layer(axum::middleware::from_fn_with_state(
            api_state.clone(),
            check_auth_middleware,
        ))
        .route("/page/:number/publish", post(publish_page_route))
        .route("/page/:number", put(upsert_page_route))
        .route("/page/:number", delete(delete_page_route));

    // Define routes
    let app = Router::new()
        .with_state(api_state)
        .nest("/", private_routes)
        .route("/", get(root_route))
        .route("/page", get(list_pages_route))
        .route("/page/:number", get(page_detail_route))
        .route("/chapter", get(list_chapters_route))
        .route("/chapter/:number", get(chapter_detail_route));

    // Determine port
    let port = std::env::var("PORT").unwrap_or("3000".into());

    // Start server
    axum::Server::bind(&format!("0.0.0.0:{port}").parse().unwrap())
        .serve(app.into_make_service_with_connect_info::<SocketAddr>())
        .with_graceful_shutdown(async {
            tokio::signal::ctrl_c()
                .await
                .expect("Failed to install Ctrl+C handler")
        })
        .await
        .unwrap();
}

/// Check that authorization header is valid
async fn check_auth_middleware<B>(
    TypedHeader(auth): TypedHeader<Authorization<Bearer>>,
    State(api_state): State<ApiState>,
    request: Request<B>,
    next: Next<B>,
) -> Result<Response, StatusCode> {
    if auth.token() == api_state.api_token {
        let response = next.run(request).await;
        Ok(response)
    } else {
        Err(StatusCode::UNAUTHORIZED)
    }
}

async fn root_route() -> &'static str {
    "Solar and Sundry Api\n"
}

async fn publish_page_route() -> &'static str {
    "publish"
}

async fn upsert_page_route() -> &'static str {
    "upsert"
}

async fn delete_page_route() -> &'static str {
    "delete"
}

async fn list_pages_route() -> &'static str {
    "list pages"
}

async fn page_detail_route() -> &'static str {
    "page detail"
}

async fn list_chapters_route() -> &'static str {
    "list chapters"
}

async fn chapter_detail_route() -> &'static str {
    "chapter detail"
}
