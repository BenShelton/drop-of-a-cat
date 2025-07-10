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
        <>
            <header class="max-w-lg mx-auto">
                <a href="#">
                    <h1 class="text-4xl font-bold text-white text-center">{ "Drop Of A Cat" }</h1>
                </a>
            </header>

            <main class="bg-white max-w-lg mx-auto p-8 md:p-12 my-10 rounded-lg shadow-2xl">
                <section>
                    <h3 class="font-bold text-2xl">{ "Welcome!" }</h3>
                    <p class="text-gray-600 pt-2">{ "Sign in to access events." }</p>
                </section>

                <section class="mt-10">
                    <form class="flex flex-col">
                        <div class="mb-6 pt-3 rounded bg-gray-200">
                            <label class="block text-gray-700 text-sm font-bold mb-2 ml-3" for="email">{ "Full Name" }</label>
                            <input
                                type="text"
                                id="email"
                                placeholder="Your name helps others recognize you"
                                class="bg-gray-200 rounded w-full text-gray-700 focus:outline-none border-b-4 border-gray-300 focus:border-purple-600 transition duration-500 px-3 pb-3"
                                value={(*name).clone()}
                                oninput={Callback::from(move |e: InputEvent| {
                                    let input: HtmlInputElement = e.target_unchecked_into();
                                    name.set(input.value());
                                })}
                             />
                        </div>
                        <div class="mb-6 pt-3 rounded bg-gray-200">
                            <label
                                class="block text-gray-700 text-sm font-bold mb-2 ml-3"
                                for="password"
                            >{ "Password" }</label>
                            <input
                                type="password"
                                id="password"
                                placeholder="Enter the website password you received"
                                class="bg-gray-200 rounded w-full text-gray-700 focus:outline-none border-b-4 border-gray-300 focus:border-purple-600 transition duration-500 px-3 pb-3"
                                value={(*password).clone()}
                                oninput={Callback::from(move |e: InputEvent| {
                                    let input: HtmlInputElement = e.target_unchecked_into();
                                    password.set(input.value());
                                })}
                            />
                        </div>
                        <button
                            class="bg-purple-600 hover:bg-purple-700 text-white font-bold py-2 rounded shadow-lg hover:shadow-xl transition duration-200"
                            type="submit"
                            onclick={on_login}
                        >{ "Sign In" }</button>
                    </form>
                </section>
            </main>
        </>
    }
}
