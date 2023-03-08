use data::{CreatePage, Page};
use serde_json::json;
use worker::{kv::KvStore, *};

mod data;
mod utils;

const KV_NAMESPACE: &str = "SOLAR_AND_SUNDRY";

type RouteContext = worker::RouteContext<KvStore>;

#[event(fetch)]
pub async fn main(req: Request, env: Env, _ctx: worker::Context) -> Result<Response> {
    // Generic utils
    utils::log_request(&req);
    utils::set_panic_hook();

    // Get keyv store
    let store = env.kv(KV_NAMESPACE)?;

    // Setup routes
    let router = Router::with_data(store);
    router
        .get("/", |_, _| Response::ok("SaS worker api"))
        .put_async("/page", upsert_page_route)
        .post_async("/page/:number/publish", publish_page_route)
        .delete_async("/page/:number", delete_page_route)
        .get_async("/page/:number", get_page_route)
        .get_async("/chapter", get_all_chapters_route)
        .get_async("/chapter/:number", get_chapter_route)
        .run(req, env)
        .await
}

async fn get_page(ctx: &RouteContext) -> std::result::Result<Page, String> {
    // Parse page number
    let Ok(page_number) = ctx.param("number").unwrap().parse::<usize>() else {
        return Err("Expected numeric path parameter 'number'".to_owned());
    };

    // Find page
    let Ok(Some(page)) = Page::get_by_number(&ctx.data, page_number).await else {
        return Err(format!("Failed to find page number {}", page_number));
    };

    Ok(page)
}

async fn upsert_page_route(mut req: Request, ctx: RouteContext) -> Result<Response> {
    // Parse input as create page
    let create_page = match req.json::<CreatePage>().await {
        Ok(page) => page,
        Err(error) => return Response::error(format!("Failed to parse page: {}", error), 400),
    };

    // Perform upsert
    let page: Page = create_page.into();
    page.save(&ctx.data).await?;

    // Return okay
    Response::from_json(&page).map(|r| r.with_status(201))
}

async fn publish_page_route(_req: Request, ctx: RouteContext) -> Result<Response> {
    // Get page
    let mut page = match get_page(&ctx).await {
        Ok(page) => page,
        Err(error) => return Response::error(error, 400),
    };

    // Update page
    page.is_published = true;
    page.save(&ctx.data).await?;

    // Return page
    Response::from_json(&page)
}

async fn delete_page_route(_req: Request, ctx: RouteContext) -> Result<Response> {
    // Get page
    let page = match get_page(&ctx).await {
        Ok(page) => page,
        Err(error) => return Response::error(error, 400),
    };

    // Delete page and return result
    page.delete(&ctx.data).await?;
    Response::from_json(&json!({ "deleted": true, "page_number": page.page_number }))
}

async fn get_page_route(_req: Request, ctx: RouteContext) -> Result<Response> {
    // Get page
    let page = match get_page(&ctx).await {
        Ok(page) => page,
        Err(error) => return Response::error(error, 400),
    };

    // Return page
    Response::from_json(&page)
}

async fn get_chapter_route(req: Request, ctx: RouteContext) -> Result<Response> {
    Response::ok("todo!")
}

async fn get_all_chapters_route(req: Request, ctx: RouteContext) -> Result<Response> {
    Response::ok("todo!")
}
