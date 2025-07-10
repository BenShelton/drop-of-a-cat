use dto::Video;
use gloo_net::http::Request;
use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    let video = use_state(|| None);
    {
        let video = video.clone();
        use_effect_with((), move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_video: Video = Request::get("/api/hello")
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();
                video.set(Some(fetched_video));
            });
            || ()
        });
    }

    html! {
        <main>
            <img class="logo" src="https://yew.rs/img/logo.svg" alt="Yew logo" />
            <h1>{ "Hello World!" }</h1>
            <span class="subtitle">{ "from Yew with " }<i class="heart" /></span>
            <div class="video">
                {
                    if let Some(video) = (*video).clone() {
                        html! {
                            <>
                                <h2>{ &video.title }</h2>
                                <p>{ format!("Speaker: {}", &video.speaker) }</p>
                            </>
                        }
                    } else {
                        html! { <p>{ "Loading video..." }</p> }
                    }
                }
            </div>
        </main>
    }
}
