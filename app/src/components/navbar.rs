use gloo_storage::{LocalStorage, Storage};
use yew::prelude::*;
use yew_router::prelude::*;

use crate::{components::theme_controller::ThemeController, router::Route};

#[function_component(Navbar)]
pub fn navbar() -> Html {
    let navigator = use_navigator().unwrap();
    let on_logout = {
        Callback::from(move |_: MouseEvent| {
            LocalStorage::clear();
            navigator.push(&Route::Login);
        })
    };

    let route = use_route::<Route>().unwrap();
    let logged_in = use_memo(route, |_| {
        !LocalStorage::get::<String>("token")
            .unwrap_or_default()
            .is_empty()
    });

    html! {
        <div class="navbar bg-base-100 shadow-sm">
            <div class="flex-1">
                <Link<Route> to={Route::Home} classes="btn btn-ghost text-xl">{ "Drop Of A Cat" }</Link<Route>>
            </div>
            <div class="flex-none">
                <ThemeController />
            </div>
            {
                if *logged_in {
                    html! {
                        <div class="flex-none">
                            <btn class="btn" onclick={on_logout}>{ "Logout" }</btn>
                        </div>
                    }
                } else {
                    html! {}
                }
            }
        </div>
    }
}
