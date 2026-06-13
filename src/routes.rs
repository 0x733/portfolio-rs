use axum::response::IntoResponse;
use maud::Markup;
use crate::templates;

pub async fn home_handler() -> impl IntoResponse {
    let markup: Markup = templates::layout::render("/", "Hüseyin | Portfolio", templates::home::view());
    markup
}

pub async fn about_handler() -> impl IntoResponse {
    let markup: Markup = templates::layout::render("/about", "About | Hüseyin", templates::about::view());
    markup
}

pub async fn project_handler() -> impl IntoResponse {
    let markup: Markup = templates::layout::render("/project", "Projects | Hüseyin", templates::project::view());
    markup
}
