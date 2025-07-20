use yew::prelude::*;
use yew_router::prelude::*;

mod guards;

use crate::{
    pages::{event::EventPage, home::HomePage, login::LoginPage, not_found::NotFoundPage},
    router::guards::authentication_guard,
};

#[derive(Debug, Clone, PartialEq, Routable)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/login")]
    Login,
    #[at("/event/:uuid")]
    Event { uuid: String },
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => authentication_guard(html! { <HomePage /> }),
        Route::Login => html! { <LoginPage /> },
        Route::Event { .. } => html! { <EventPage /> },
        Route::NotFound => html! { <NotFoundPage /> },
    }
}
