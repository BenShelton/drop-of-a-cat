use dto::{
    api::{APIError, SuggestionRequest, SuggestionResponse},
    collections::Event,
};
use gloo_net::http::Request;
use gloo_storage::{LocalStorage, Storage};
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::hooks::use_navigator;

use crate::{components::back_btn::BackBtn, router::Route};

#[function_component(SuggestionPage)]
pub fn suggestion_page() -> Html {
    let title = use_state(|| String::new());
    let description = use_state(|| String::new());
    let date = use_state(|| String::new());
    let time = use_state(|| String::new());
    let location = use_state(|| String::new());
    let organiser = use_state(|| String::new());

    let loading = use_state(|| false);
    let error_message = use_state(|| String::new());
    let navigator = use_navigator().expect("Navigator should be available");

    let missing_info =
        { title.is_empty() || description.is_empty() || date.is_empty() || organiser.is_empty() };
    let disabled_btn = { *loading || missing_info };

    let on_submit = {
        let title = title.clone();
        let description = description.clone();
        let date = date.clone();
        let time = time.clone();
        let location = location.clone();
        let organiser = organiser.clone();
        let loading = loading.clone();
        let error_message = error_message.clone();
        let navigator = navigator.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            let error_message = error_message.clone();
            error_message.set(String::new());
            if missing_info {
                return;
            }
            let title = title.clone();
            let description = description.clone();
            let date = date.clone();
            let time = time.clone();
            let location = location.clone();
            let organiser = organiser.clone();
            let loading = loading.clone();
            let navigator = navigator.clone();
            wasm_bindgen_futures::spawn_local(async move {
                loading.set(true);
                let result = Request::post("/api/suggestion")
                    .json(&SuggestionRequest {
                        event: Event {
                            title: (*title).clone(),
                            description: (*description).clone(),
                            date: (*date).clone(),
                            time: (*time).clone(),
                            location: (*location).clone(),
                            organiser: (*organiser).clone(),
                            ..Default::default()
                        },
                    })
                    .unwrap()
                    .send()
                    .await
                    .unwrap();
                if !result.ok() {
                    let err = result.json::<APIError>().await.unwrap().message.to_string();
                    error_message.set(err);
                    loading.set(false);
                    return;
                }
                let result = result.json::<SuggestionResponse>().await.unwrap();
                match result.result {
                    true => {
                        LocalStorage::set("toast", "Thank you for the suggestion!".to_string())
                            .ok();
                        navigator.push(&Route::Home);
                    }
                    false => {
                        error_message.set("Suggestion was not saved, please try again.".to_string())
                    }
                }
                loading.set(false);
            })
        })
    };

    html! {
        <section class="min-h-screen p-6">
            <BackBtn />
            <div class="max-w-md">
                <h1 class="text-xl font-bold mb-4">
                    { "Suggest An Event" }
                </h1>
            </div>
            <fieldset class="fieldset bg-base-200 border-base-300 rounded-box max-w-md border p-4">
                <legend class="fieldset-legend">{ "Please fill out the following details" }</legend>

                <label class="label">{ "Title" }</label>
                <input
                    type="text"
                    class="input mb-4"
                    placeholder="The name of the event"
                    value={(*title).clone()}
                    oninput={Callback::from(move |e: InputEvent| {
                        let input: HtmlInputElement = e.target_unchecked_into();
                        title.set(input.value());
                    })}
                />

                <label class="label">{ "Description" }</label>
                <textarea
                    class="textarea h-24 mb-4"
                    placeholder="A brief description of the event"
                    value={(*description).clone()}
                    oninput={Callback::from(move |e: InputEvent| {
                        let input: HtmlInputElement = e.target_unchecked_into();
                        description.set(input.value());
                    })}
                ></textarea>

                <label class="label">{ "Date" }</label>
                <input
                    type="date"
                    class="input mb-4"
                    placeholder="The date of the event"
                    value={(*date).clone()}
                    oninput={Callback::from(move |e: InputEvent| {
                        let input: HtmlInputElement = e.target_unchecked_into();
                        date.set(input.value());
                    })}
                />

                <label class="label">{ "Time (optional)" }</label>
                <input
                    type="time"
                    class="input mb-4"
                    placeholder="The time of the event"
                    value={(*time).clone()}
                    oninput={Callback::from(move |e: InputEvent| {
                        let input: HtmlInputElement = e.target_unchecked_into();
                        time.set(input.value());
                    })}
                />

                <label class="label">{ "Location (optional)" }</label>
                <input
                    class="input mb-4"
                    placeholder="Where the event will take place"
                    value={(*location).clone()}
                    oninput={Callback::from(move |e: InputEvent| {
                        let input: HtmlInputElement = e.target_unchecked_into();
                        location.set(input.value());
                    })}
                />

                <label class="label">{ "Organiser" }</label>
                <input
                    class="input mb-4"
                    placeholder="Who is organising the event"
                    value={(*organiser).clone()}
                    oninput={Callback::from(move |e: InputEvent| {
                        let input: HtmlInputElement = e.target_unchecked_into();
                        organiser.set(input.value());
                    })}
                />

                <button class="btn btn-primary mt-4" disabled={disabled_btn} onclick={on_submit}>
                    {
                        if *loading {
                            html! {
                                <span class="loading loading-spinner"></span>
                            }
                        } else {
                            html! {}
                        }
                    }
                    { "Submit" }
                </button>

                <p class="mt-4 text-error">{ (*error_message).clone() }</p>
            </fieldset>
        </section>
    }
}
