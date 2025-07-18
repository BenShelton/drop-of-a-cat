use gloo_storage::{LocalStorage, Storage};
use yew::prelude::*;
use yew_router::prelude::*;

use crate::router::Route;

/// Checks that the user is authenticated by checking for a token in local storage.
/// - If the token is present and not empty, it returns the provided HTML content.
/// - If the token is not present or empty, it redirects to the login page.
pub fn authentication_guard(html: Html) -> Html {
    let token = LocalStorage::get::<String>("token");
    match token {
        Ok(token) if !token.is_empty() => html,
        _ => html! { <Redirect<Route> to={Route::Login}/> },
    }
}
