mod migrations;

use cot::bytes::Bytes;
use cot::cli::CliMetadata;
use cot::db::migrations::SyncDynMigration;
use cot::form::{FormContext, FormResult};
use cot::middleware::{AuthMiddleware, LiveReloadMiddleware, SessionMiddleware};
use cot::project::{MiddlewareContext, RegisterAppsContext, RootHandlerBuilder};
use cot::request::Request;
use cot::response::{Response, ResponseExt};
use cot::reverse_redirect;
use cot::router::{Route, Router};
use cot::static_files::StaticFilesMiddleware;
use cot::Method;
use cot::{static_files, App, AppBuilder, Body, BoxedHandler, Project, StatusCode};
use rinja::Template;
use std::fmt::Display;
struct StocksApp;

struct Item {
    title: String,
}
impl Display for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.title)
    }
}

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    item: Item,
}

async fn index() -> cot::Result<Response> {
    let index_template = IndexTemplate {
        item: Item {
            title: String::from("bar"),
        },
    };
    let rendered = index_template.render()?;

    Ok(Response::new_html(StatusCode::OK, Body::fixed(rendered)))
}
use cot::form::Form;

#[derive(Form, Debug)]
struct ContactForm {
    name: String,
    email: String,
    #[form(opt(max_length = 1000))]
    message: String,
}
impl Display for ContactForm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

#[derive(Template, Debug)]
#[template(path = "form.html")]
struct ContactTemplate {
    request: &'static cot::request::Request,
    form: ContactForm,
}

async fn contact(mut request: Request) -> cot::Result<Response> {
    // Handle POST request (form submission)
    if request.method() == Method::POST {
        match ContactForm::from_request(&mut request).await? {
            FormResult::Ok(form) => {
                // Form is valid! Process the data
                println!("Message from {}: {}", form.name, form.message);

                // Redirect after successful submission
                Ok(reverse_redirect!(request, "thank_you")?)
            }
            FormResult::ValidationError(context) => {
                // Form has errors - render the template with error messages
                let template = ContactTemplate {
                    request: &request,
                    form: context,
                };
                Ok(Response::new_html(
                    StatusCode::OK,
                    Body::fixed(template.render()?),
                ))
            }
        }
    } else {
        // Handle GET request (display empty form)
        let template = ContactTemplate {
            request: &request,
            form: ContactForm::build_context(&mut request).await?,
        };

        Ok(Response::new_html(
            StatusCode::OK,
            Body::fixed(template.render()?),
        ))
    }
}

impl App for StocksApp {
    fn name(&self) -> &'static str {
        env!("CARGO_CRATE_NAME")
    }

    fn migrations(&self) -> Vec<Box<SyncDynMigration>> {
        cot::db::migrations::wrap_migrations(migrations::MIGRATIONS)
    }

    fn router(&self) -> Router {
        Router::with_urls([
            Route::with_handler_and_name("/", index, "list_stocks"),
            Route::with_handler_and_name("/submit", contact, "submit_contact"),
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
