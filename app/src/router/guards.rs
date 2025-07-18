use gloo_storage::{LocalStorage, Storage};
use yew::prelude::*;
use yew_router::prelude::*;

use crate::router::Route;

pub fn authentication_guard(html: Html) -> Html {
    let token = LocalStorage::get::<String>("token");
    match token {
        Ok(token) if !token.is_empty() => html,
        _ => html! { <Redirect<Route> to={Route::Login}/> },
    }
}
