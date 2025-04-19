mod migrations;

use cot::bytes::Bytes;
use cot::cli::CliMetadata;
use cot::db::migrations::SyncDynMigration;
use cot::middleware::{AuthMiddleware, LiveReloadMiddleware, SessionMiddleware};
use cot::project::{MiddlewareContext, RegisterAppsContext, RootHandlerBuilder};
use cot::request::extractors::Path;
use cot::response::{Response, ResponseExt};
use cot::router::{Route, Router};
use cot::static_files::StaticFilesMiddleware;
use cot::{static_files, App, AppBuilder, Body, BoxedHandler, Project, StatusCode};
use rinja::Template;

#[derive(Debug, Template)]
#[template(path = "index.html")]
struct IndexTemplate {}

async fn index() -> cot::Result<Response> {
    let index_template = IndexTemplate {};
    let rendered = index_template.render()?;

    Ok(Response::new_html(StatusCode::OK, Body::fixed(rendered)))
}
async fn hello() -> cot::Result<Response> {
    Ok(Response::new_html(
        StatusCode::OK,
        Body::fixed("Hello World!"),
    ))
}
async fn hello_name(
    Path((first_name, last_name)): Path<(String, String)>,
) -> cot::Result<Response> {
    Ok(Response::new_html(
        StatusCode::OK,
        Body::fixed(format!("Hello, {}, {}!", first_name, last_name)),
    ))
}
struct StocksApp;

impl App for StocksApp {
    fn name(&self) -> &'static str {
        env!("CARGO_CRATE_NAME")
    }

    fn migrations(&self) -> Vec<Box<SyncDynMigration>> {
        cot::db::migrations::wrap_migrations(migrations::MIGRATIONS)
    }

    fn router(&self) -> Router {
        Router::with_urls([
            Route::with_handler_and_name("/", index, "index"),
            Route::with_handler_and_name("/hello", hello, "hello"),
            Route::with_handler_and_name(
                "/hello/{first_name}/{last_name}/",
                hello_name,
                "hello_name",
            ),
        ])
    }

    fn static_files(&self) -> Vec<(String, Bytes)> {
        static_files!("css/main.css")
    }
}

struct StocksProject;

impl Project for StocksProject {
    fn cli_metadata(&self) -> CliMetadata {
        cot::cli::metadata!()
    }

    fn register_apps(&self, apps: &mut AppBuilder, _context: &RegisterAppsContext) {
        apps.register_with_views(StocksApp, "");
    }

    fn middlewares(
        &self,
        handler: RootHandlerBuilder,
        context: &MiddlewareContext,
    ) -> BoxedHandler {
        handler
            .middleware(StaticFilesMiddleware::from_context(context))
            .middleware(AuthMiddleware::new())
            .middleware(SessionMiddleware::new())
            .middleware(LiveReloadMiddleware::from_context(context))
            .build()
    }
}

#[cot::main]
fn main() -> impl Project {
    StocksProject
}
