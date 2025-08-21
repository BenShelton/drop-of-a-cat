use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use dto::collections::Event;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::{components::countdown::Countdown, router::Route};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub event: Event,
}

#[function_component(EventCard)]
pub fn event_card(props: &Props) -> Html {
    let date = NaiveDateTime::new(
        NaiveDate::parse_from_str(&props.event.date, "%Y-%m-%d").unwrap_or_default(),
        NaiveTime::parse_from_str(&props.event.time, "%H:%M:%S").unwrap_or_default(),
    );
    let past = date < chrono::Local::now().naive_local();
    let in_progress = past && date.date() == chrono::Local::now().naive_local().date();

    html! {
        <div class="card bg-secondary text-secondary-content w-96">
            <div class="card-body">
                <h2 class="card-title">{ &props.event.title }</h2>
                {
                    if past {
                        if in_progress {
                            html! { <span class="badge badge-info">{ "In Progress" }</span> }
                        } else {
                            html! { <span class="badge badge-info">{ "Past Event" }</span> }
                        }
                    } else {
                        html! {
                            <Countdown
                                date={props.event.date.clone()}
                                time={props.event.time.clone()}
                            />
                        }
                    }
                }
            </div>
            <div class="card-actions justify-end p-4">
                <Link<Route> to={Route::Event { uuid: props.event.uuid.clone() }} classes="btn btn-primary">{ "Learn More" }</Link<Route>>
            </div>
        </div>
    }
}
