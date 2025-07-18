use yew::prelude::*;
use yew_router::prelude::Link;

use crate::router::Route;

#[function_component(NotFoundPage)]
pub fn not_found_page() -> Html {
    html! {
        <section class="hero min-h-screen">
            <div class="hero-content">
                <div class="max-w-md text-center">
                    <h1 class="text-5xl font-bold">
                        { "Page not found" }
                    </h1>
                    <h2 class="py-6">
                        { "Page does not seem to exist" }
                    </h2>
                    <Link<Route> to={Route::Home} classes="link link-primary">{ "Return to Home" }</Link<Route>>
                </div>
            </div>
        </section>
    }
}
