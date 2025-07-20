use dto::collections::Event;
use yew::prelude::*;
use yew_icons::{Icon, IconId};

use crate::components::event_card::EventCard;

#[function_component(HomePage)]
pub fn home_page() -> Html {
    let events = vec![
        Event {
            uuid: "1".to_string(),
            title: "Event 1".to_string(),
            description: "This is an event".to_string(),
            date: "2025-01-01".to_string(),
            time: "10:00am".to_string(),
            location: "None".to_string(),
            organizer: "Ben Jones".to_string(),
            ..Default::default()
        },
        Event {
            uuid: "2".to_string(),
            title: "Event 2".to_string(),
            description: "This is an event".to_string(),
            date: "2025-01-01".to_string(),
            time: "10:00am".to_string(),
            location: "None".to_string(),
            organizer: "Ben Jones".to_string(),
            ..Default::default()
        },
    ];

    html! {
        <section class="min-h-screen p-6">
            <div class="max-w-md">
                <h1 class="text-xl font-bold mb-4">
                    { "Upcoming Events" }
                </h1>
            </div>
            <div class="grid grid-cols-4 gap-4">
                {
                    events.into_iter().map(|event| {
                        let title = event.title.clone();
                        html! { <EventCard key={title} event={event} /> }
                    }).collect::<Html>()
                }
            </div>
            <div class="tooltip tooltip-open tooltip-left tooltip-accent fixed bottom-4 right-4" data-tip="Suggest an event!">
                <button class="btn btn-circle btn-xl btn-accent">
                    <Icon icon_id={IconId::HeroiconsSolidLightBulb}/>
                </button>
            </div>
        </section>
    }
}
