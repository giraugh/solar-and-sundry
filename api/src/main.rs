mod data;

use data::{
    chapter::Chapter,
    dto::{ChapterResponse, CreatePage, PageResponse},
    page::Page,
};

use axum::{
    extract::{Path, State},
    headers::authorization::{Authorization, Bearer},
    http::{
        header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
        Method, Request, StatusCode,
    },
    middleware::Next,
    response::{IntoResponse, Response},
    routing::{delete, get, post, put},
    Json, Router, TypedHeader,
};
use dotenvy::dotenv;
use libsql::Database;
use std::{env, net::SocketAddr, sync::Arc};
use tokio::sync::Mutex;
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing::{info, level_filters::LevelFilter};
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

pub type DbRef = Arc<Mutex<libsql::Connection>>;

#[derive(Clone)]
struct ApiState {
    pub api_token: String,
    pub db_conn: DbRef,
}

#[derive(Debug)]
struct ApiError(anyhow::Error);

impl From<anyhow::Error> for ApiError {
    fn from(value: anyhow::Error) -> Self {
        ApiError(value)
    }
}

type ApiResult<T> = Result<Json<T>, ApiError>;

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong: {}", self.0),
        )
            .into_response()
    }
}

#[tokio::main]
async fn main() {
    // Init environment
    dotenv().ok();

    // Init logging
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(
            EnvFilter::builder()
                .with_default_directive(LevelFilter::INFO.into())
                .from_env_lossy(),
        )
        .init();

    // CORS configuration
    let cors = CorsLayer::very_permissive()
        .allow_credentials(true)
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE])
        .allow_methods([Method::PUT, Method::GET, Method::POST, Method::DELETE]);

    // Determine db url
    let db = if let Ok(db_url) = std::env::var("DB_URL") {
        let db_auth = std::env::var("DB_AUTH_TOKEN").expect("DB_AUTH_TOKEN to be present");
        info!("Connecting to remote libsql db at {db_url}");
        Database::open_remote(db_url, db_auth).unwrap()
    } else {
        let db_path = "file:///tmp/sas.db".to_owned();
        info!("Using local libsql db at {db_path}");
        Database::open(db_path).unwrap()
    };

    // Connect to db
    let db_conn = db.connect().unwrap();
    let db_conn = Arc::new(Mutex::new(db_conn));

    // Create db tables if they don't exist
    use data::page::Page;
    create_tables![db_conn; Page];

    // Init api state
    let api_state = ApiState {
        api_token: env::var("API_TOKEN").expect("Expected API_TOKEN environment variable"),
        db_conn,
    };

    // Define private routes
    let private_routes = Router::new()
        .layer(axum::middleware::from_fn_with_state(
            api_state.clone(),
            check_auth_middleware,
        ))
        .route("/page/:number/publish", post(publish_page_route))
        .route("/page/:number", delete(delete_page_route))
        .route("/page", put(upsert_page_route));

    // Define routes
    let app = Router::new()
        .nest("/", private_routes)
        .route("/", get(root_route))
        .route("/page", get(list_pages_route))
        .route("/page/:number", get(page_detail_route))
        .route("/chapter", get(list_chapters_route))
        .route("/chapter/:number", get(chapter_detail_route))
        .with_state(api_state)
        .layer(cors)
        .layer(TraceLayer::new_for_http());

    // Determine port
    let port = std::env::var("PORT").unwrap_or("3000".into());

    // Start server
    info!("Starting http server on port {port}");
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

async fn publish_page_route(
    State(api_state): State<ApiState>,
    Path(page_number): Path<usize>,
) -> ApiResult<String> {
    Page::set_published(api_state.db_conn, page_number, true).await?;
    info!("Published page {page_number}");
    Ok(Json("Published".into()))
}

async fn upsert_page_route(
    State(api_state): State<ApiState>,
    Json(create_page): Json<CreatePage>,
) -> ApiResult<String> {
    info!("Upserting page {}", &create_page.page_number);
    Page::upsert(api_state.db_conn, create_page.into()).await?;
    Ok(Json("Created".into()))
}

async fn delete_page_route(
    State(api_state): State<ApiState>,
    Path(page_number): Path<usize>,
) -> ApiResult<String> {
    Page::delete(api_state.db_conn, page_number).await?;
    info!("Deleted page {page_number}");
    Ok(Json("Deleted".into()))
}

async fn list_pages_route(State(api_state): State<ApiState>) -> ApiResult<Vec<PageResponse>> {
    let pages = Page::get_all_published(api_state.db_conn)
        .await?
        .into_iter()
        .map(Into::into)
        .collect();
    Ok(Json(pages))
}

async fn page_detail_route(
    State(api_state): State<ApiState>,
    Path(page_number): Path<usize>,
) -> ApiResult<Option<PageResponse>> {
    let page = Page::get(api_state.db_conn, page_number)
        .await?
        .map(Into::into);
    // TODO: Ideally, this should also return a 404 if none
    Ok(Json(page))
}

async fn list_chapters_route(State(api_state): State<ApiState>) -> ApiResult<Vec<ChapterResponse>> {
    let chapters = Chapter::get_all(api_state.db_conn)
        .await?
        .into_iter()
        .map(Into::into)
        .collect();
    Ok(Json(chapters))
}

async fn chapter_detail_route(
    State(api_state): State<ApiState>,
    Path(chapter_number): Path<usize>,
) -> ApiResult<Option<ChapterResponse>> {
    let chapter = Chapter::get(api_state.db_conn, chapter_number)
        .await?
        .map(Into::into);
    // TODO: Ideally, this should also return a 404 if none
    Ok(Json(chapter))
}
