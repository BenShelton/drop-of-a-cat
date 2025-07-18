use yew::prelude::*;
use yew_router::prelude::*;

mod guards;

use crate::{
    pages::{home::HomePage, login::LoginPage, not_found::NotFoundPage},
    router::guards::authentication_guard,
};

#[derive(Debug, Clone, Copy, PartialEq, Routable)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/login")]
    Login,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => authentication_guard(html! { <HomePage /> }),
        Route::Login => html! { <LoginPage /> },
        Route::NotFound => html! { <NotFoundPage /> },
    }
}
