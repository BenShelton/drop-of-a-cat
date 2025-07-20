use dto::collections::Event;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::router::Route;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub event: Event,
}

#[function_component(EventCard)]
pub fn event_card(props: &Props) -> Html {
    html! {
        <div class="card bg-secondary text-secondary-content w-96">
            <div class="card-body">
                <h2 class="card-title">{ &props.event.title }</h2>
                <p>{ &props.event.description }</p>
                <div class="card-actions justify-end">
                    <Link<Route> to={Route::Event { uuid: props.event.uuid.clone() }} classes="btn btn-primary">{ "Learn More" }</Link<Route>>
                </div>
            </div>
        </div>
    }
}
