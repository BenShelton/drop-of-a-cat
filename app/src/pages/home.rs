use dto::api::HomeResponse;
use gloo_net::http::Request;
use gloo_storage::{LocalStorage, Storage};
use yew::{prelude::*, suspense::use_future};
use yew_icons::{Icon, IconId};
use yew_router::prelude::*;

use crate::{components::event_card::EventCard, router::Route};

#[function_component(Content)]
fn content() -> HtmlResult {
    let toast = LocalStorage::get::<String>("toast");
    LocalStorage::delete("toast");

    let res = use_future(|| async {
        let result = Request::get("/api/home")
            .send()
            .await
            .map_err(|_| "Failed to fetch events".to_string())?;
        if !result.ok() {
            return Err("Failed to fetch events".to_string());
        }
        let response = result
            .json::<HomeResponse>()
            .await
            .map_err(|_| "Failed to fetch events".to_string())?;
        Ok(response.events)
    })?;

    match *res {
        Ok(ref events) => Ok(html! {
            <>
                <div class="max-w-md">
                    <h1 class="text-xl font-bold mb-4">
                        { "Upcoming Events" }
                    </h1>
                </div>
                <div class="flex gap-4">
                    {
                        events.iter().map(|event| {
                            let title = event.title.clone();
                            html! { <EventCard key={title} event={event.clone()} /> }
                        }).collect::<Html>()
                    }
                </div>
                <div class="tooltip tooltip-open tooltip-left tooltip-accent fixed bottom-4 right-4" data-tip="Suggest an event!">
                     <Link<Route> to={Route::Suggestion} classes="btn btn-circle btn-xl btn-accent">
                        <Icon icon_id={IconId::HeroiconsSolidLightBulb}/>
                    </Link<Route>>
                </div>
                {
                    if let Ok(message) = toast {
                        html! {
                            <div class="toast toast-start">
                                <div class="alert alert-success">
                                    <div>
                                        <span>{ message }</span>
                                    </div>
                                </div>
                            </div>
                        }
                    } else {
                        html! {}
                    }
                }
            </>
        }),
        Err(_) => Ok({
            html! { "An error occurred" }
        }),
    }
}

#[function_component(HomePage)]
pub fn home_page() -> Html {
    let fallback = html! {
        <div class="max-w-md flex">
            <span class="loading loading-spinner"></span>
            <h1 class="text-xl font-bold mb-4">
                { "Loading..." }
            </h1>
        </div>
    };

    html! {
        <section class="min-h-screen p-6">
            <Suspense {fallback}>
                <Content />
            </Suspense>
        </section>
    }
}
