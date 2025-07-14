#![allow(clippy::redundant_closure)]

use dto::{LoginRequest, LoginResponse};
use gloo_net::http::Request;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    let name = use_state(|| String::new());
    let password = use_state(|| String::new());
    let on_login = {
        let name = name.clone();
        let password = password.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            let name = name.clone();
            let password = password.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let result = Request::post("/api/login")
                    .json(&LoginRequest {
                        name: (*name).clone(),
                        password: (*password).clone(),
                    })
                    .unwrap()
                    .send()
                    .await
                    .unwrap();
                if !result.ok() {
                    web_sys::console::error_1(&format!("Error: {}", result.status()).into());
                    return;
                }
                let result = result.json::<LoginResponse>().await.unwrap();
                web_sys::console::log_1(&result.result.into());
            })
        })
    };
    html! {
        <div class="hero bg-base-200 min-h-screen">
            <div class="hero-content flex-col">
                <div class="text-center">
                    <h1 class="text-5xl font-bold">{ "Drop Of A Cat" }</h1>
                    <p class="py-6">{ "Sign in to access events." }</p>
                </div>
                <div class="card bg-base-100 w-full max-w-sm shrink-0 shadow-2xl">
                    <div class="card-body">
                        <fieldset class="fieldset">
                            <p>{ "Your name helps others recognize you" }</p>
                            <label class="label">{ "Full Name" }</label>
                            <input
                                type="text"
                                class="input"
                                value={(*name).clone()}
                                oninput={Callback::from(move |e: InputEvent| {
                                    let input: HtmlInputElement = e.target_unchecked_into();
                                    name.set(input.value());
                                })}
                            />
                            <p class="mt-6">{ "Enter the website password you received" }</p>
                            <label class="label">{ "Password" }</label>
                            <input
                                type="password"
                                class="input"
                                value={(*password).clone()}
                                oninput={Callback::from(move |e: InputEvent| {
                                    let input: HtmlInputElement = e.target_unchecked_into();
                                    password.set(input.value());
                                })}
                                />
                            <button class="btn btn-neutral mt-4" onclick={on_login}>{ "Login" }</button>
                        </fieldset>
                    </div>
                </div>
            </div>
        </div>
    }
}
