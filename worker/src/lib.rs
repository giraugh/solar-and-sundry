use data::{Chapter, CreatePageBody, Page};
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
        .get_async("/page/:number/image", get_page_image_route)
        .get_async("/chapter", get_all_chapters_route)
        .get_async("/chapter/:number", get_chapter_route)
        .run(req, env)
        .await
}

fn check_authorisation(
    req: &Request,
    ctx: &RouteContext,
) -> std::result::Result<(), Result<Response>> {
    // Get authorisation header
    let Ok(Some(authorisation)) = req.headers().get("Authorisation") else {
        return Err(Response::error("Unauthenticated", 403));
    };

    // Split authorisation
    let Some((scheme, token)) = authorisation.split_once(' ') else {
        return Err(Response::error("Malformed authorisation header", 403));
    };

    // Check scheme
    if scheme != "Basic" {
        return Err(Response::error("Unexpected authorisation scheme", 403));
    }

    // Check it matches a secret
    let invoke_secret = ctx
        .var("INVOKE_SECRET")
        .expect("Expected INVOKE_SECRET binding")
        .to_string();
    if token != invoke_secret {
        return Err(Response::error("Unauthorised", 403));
    }

    Ok(())
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
    // Check whether authorised
    if let Err(res) = check_authorisation(&req, &ctx) {
        return res;
    }

    // Parse input as create page
    let create_page = match req.json::<CreatePageBody>().await {
        Ok(page) => page,
        Err(error) => return Response::error(format!("Failed to parse page: {}", error), 400),
    };

    // Perform upsert
    let page: Page = create_page.into();
    page.save(&ctx.data).await?;

    // Return okay
    Response::from_json(&page).map(|r| r.with_status(201))
}

async fn publish_page_route(req: Request, ctx: RouteContext) -> Result<Response> {
    // Check whether authorised
    if let Err(res) = check_authorisation(&req, &ctx) {
        return res;
    }

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

async fn delete_page_route(req: Request, ctx: RouteContext) -> Result<Response> {
    // Check whether authorised
    if let Err(res) = check_authorisation(&req, &ctx) {
        return res;
    }

    // Get page
    let page = match get_page(&ctx).await {
        Ok(page) => page,
        Err(error) => return Response::error(error, 400),
    };

    // Delete page and return result
    page.delete(&ctx.data).await?;
    Response::from_json(&json!({ "deleted": true, "page_number": page.page_number }))
}

async fn get_page_route(req: Request, ctx: RouteContext) -> Result<Response> {
    // Get page
    let page = match get_page(&ctx).await {
        Ok(page) => page,
        Err(error) => return Response::error(error, 400),
    };

    match page.is_published {
        false => Response::error("Page is not published", 403),
        true => Response::from_json(&page.to_response(req.url().unwrap())),
    }
}

async fn get_page_image_route(_req: Request, ctx: RouteContext) -> Result<Response> {
    // Get page
    let page = match get_page(&ctx).await {
        Ok(page) => page,
        Err(error) => return Response::error(error, 400),
    };

    // Determine image url
    let account_hash = ctx
        .var("ACCOUNT_HASH")
        .expect("Can\t find ACCOUNT_HASH binding")
        .to_string();
    let image_url = page.image_url(&account_hash);

    // Create a request to get the image
    let mut request_init = RequestInit::new();
    let request_init = request_init.with_headers({
        let mut headers = Headers::new();
        headers.set("User-Agent", "Rust Worker").unwrap();
        headers
    });

    // Send the request
    let request = Request::new_with_init(image_url.as_ref(), request_init).unwrap();
    let response = match Fetch::Request(request).send().await {
        Ok(response) => response,
        Err(_) => return Response::error("Failed to fetch image", 500),
    };

    // Forward the request with updated headers
    Ok(response.with_headers({
        let mut headers = Headers::new();
        headers.set("User-Agent", "SaS worker").unwrap();
        headers
    }))
}

async fn get_chapter_route(req: Request, ctx: RouteContext) -> Result<Response> {
    // Parse chapter number
    let Ok(chapter_number) = ctx.param("number").unwrap().parse::<usize>() else {
        return Response::error("Expected numeric path parameter 'number'", 400);
    };

    // Find chapter
    let Ok(Some(chapter)) = Chapter::get_by_number(&ctx.data, chapter_number).await else {
        return Response::error(format!("Failed to find chapter number {}", chapter_number), 400);
    };

    Response::from_json(&chapter.to_response(req.url().unwrap()))
}

async fn get_all_chapters_route(req: Request, ctx: RouteContext) -> Result<Response> {
    // Get all chapters
    let chapters = Chapter::get_all(&ctx.data).await?;
    Response::from_json(
        &chapters
            .into_iter()
            .map(|chapter| chapter.to_response(req.url().unwrap()))
            .collect::<Vec<_>>(),
    )
}
