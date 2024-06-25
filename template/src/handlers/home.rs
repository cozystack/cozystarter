use std::sync::Arc;

use axum::{extract::State, http::StatusCode, response::Html};
use minijinja::context;

use crate::app::App;

pub(crate) async fn home(State(app): State<Arc<App>>) -> Result<Html<String>, StatusCode> {
    let template = app.env.get_template("home.html").unwrap();
    let rendered = template
        .render(context! {
            title => "Home",
            welcome_text => "Hello World!",
        })
        .unwrap();

    Ok(Html(rendered))
}
