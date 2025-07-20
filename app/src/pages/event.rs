use yew::prelude::*;

#[function_component(EventPage)]
pub fn event_page() -> Html {
    html! {
        <section class="min-h-screen p-6">
            <div class="max-w-md">
                <h1 class="text-xl font-bold mb-4">
                    { "Event Details" }
                </h1>
            </div>
        </section>
    }
}
