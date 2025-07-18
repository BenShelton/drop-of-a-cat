use yew::prelude::*;
use yew_router::prelude::*;

use crate::{components::theme_controller::ThemeController, router::Route};

#[function_component(Navbar)]
pub fn navbar() -> Html {
    html! {
        <div class="navbar bg-base-100 shadow-sm">
            <div class="flex-1">
                <Link<Route> to={Route::Home} classes="btn btn-ghost text-xl">{ "Drop Of A Cat" }</Link<Route>>
            </div>
            <div class="flex-none">
                <ThemeController />
            </div>
        </div>
    }
}
