#![allow(clippy::redundant_closure)]

use dto::api::{APIError, LoginRequest, LoginResponse};
use gloo_net::http::Request;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    let name = use_state(|| String::new());
    let password = use_state(|| String::new());
    let loading = use_state(|| false);
    let error_message = use_state(|| String::new());

    let missing_info = { name.is_empty() || password.is_empty() };
    let disabled_btn = { *loading || missing_info };

    let on_login = {
        let name = name.clone();
        let password = password.clone();
        let loading = loading.clone();
        let error_message = error_message.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            if missing_info {
                return;
            }
            let name = name.clone();
            let password = password.clone();
            let loading = loading.clone();
            let error_message = error_message.clone();
            wasm_bindgen_futures::spawn_local(async move {
                loading.set(true);
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
                    let err = result.json::<APIError>().await.unwrap().message.to_string();
                    error_message.set(err);
                    loading.set(false);
                    return;
                }
                let result = result.json::<LoginResponse>().await.unwrap();
                match result.result {
                    Some(user) => web_sys::console::log_1(&user.token.into()),
                    None => error_message.set("Login Failed, incorrect password.".to_string()),
                }
                loading.set(false);
            })
        })
    };

    html! {
        <div class="hero bg-base-200 min-h-screen">
            <div class="hero-content flex-col">
                <div class="text-center">
                    <h1 class="text-5xl font-bold text-primary">{ "Drop Of A Cat" }</h1>
                    <p class="py-6 text-primary">{ "Sign in to access events." }</p>
                </div>
                <div class="card bg-base-100 w-full max-w-sm shrink-0 shadow-2xl">
                    <div class="card-body">
                        <fieldset class="fieldset">
                            <label for="username" class="label">{ "Full Name" }</label>
                            <p>{ "Your name helps others recognize you" }</p>
                            <input
                                type="text"
                                id="username"
                                class="input mb-6"
                                value={(*name).clone()}
                                oninput={Callback::from(move |e: InputEvent| {
                                    let input: HtmlInputElement = e.target_unchecked_into();
                                    name.set(input.value());
                                })}
                            />
                            <label for="password" class="label">{ "Password" }</label>
                            <p>{ "Enter the website password you received" }</p>
                            <input
                                type="password"
                                id="password"
                                class="input"
                                value={(*password).clone()}
                                oninput={Callback::from(move |e: InputEvent| {
                                    let input: HtmlInputElement = e.target_unchecked_into();
                                    password.set(input.value());
                                })}
                            />
                            <button class="btn btn-primary mt-4" disabled={disabled_btn} onclick={on_login}>
                                {
                                    if *loading {
                                        html! {
                                            <span class="loading loading-spinner"></span>
                                        }
                                    } else {
                                        html! {}
                                    }
                                }
                                { "Login" }
                            </button>
                            <p class="mt-4 text-error">{ (*error_message).clone() }</p>
                        </fieldset>
                    </div>
                </div>
            </div>
        </div>
    }
}
