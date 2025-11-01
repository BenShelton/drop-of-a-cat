use dto::api::EventResponse;
use gloo_net::http::Request;
use yew::{prelude::*, suspense::use_future};

use crate::{api::get_authorization_header, components::back_btn::BackBtn};

#[derive(PartialEq, Properties)]
pub struct Props {
    pub uuid: AttrValue,
}

#[function_component(Content)]
fn content(props: &Props) -> HtmlResult {
    let request = Request::get(format!("/api/event/{}", props.uuid).as_str());
    let res = use_future(|| async {
        let result = request
            .header("Authorization", get_authorization_header().as_str())
            .send()
            .await
            .map_err(|_| "Failed to fetch event".to_string())?;
        if !result.ok() {
            return Err("Failed to fetch event".to_string());
        }
        let response = result
            .json::<EventResponse>()
            .await
            .map_err(|_| "Failed to fetch event".to_string())?;
        Ok(response.event)
    })?;

    match *res {
        Ok(ref event) => Ok(html! {
            <>
                <div class="max-w-md">
                    <h1 class="text-xl font-bold mb-4">
                        { event.title.clone() }
                    </h1>
                    <p class="mb-4">{ event.description.clone() }</p>
                    <p class="text-sm text-gray-500 mb-4">
                        { format!("Date: {}", event.date) }
                    </p>
                    <p class="text-sm text-gray-500 mb-4">
                        { format!("Location: {}", event.location) }
                    </p>
                    <p class="text-sm text-gray-500 mb-4">
                        { format!("Organiser: {}", event.organiser) }
                    </p>
                </div>
            </>
        }),
        Err(_) => Ok({
            html! { "An error occurred" }
        }),
    }
}

#[function_component(EventPage)]
pub fn event_page(props: &Props) -> Html {
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
            <BackBtn />
            <Suspense {fallback}>
                <Content uuid={props.uuid.clone()} />
            </Suspense>
        </section>
    }
}
