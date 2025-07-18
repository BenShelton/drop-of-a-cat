use yew::prelude::*;

#[function_component(HomePage)]
pub fn home_page() -> Html {
    html! {
        <section class="hero min-h-screen">
            <div class="hero-content">
                <div class="max-w-md text-center">
                    <h1 class="text-5xl font-bold">
                        { "Home" }
                    </h1>
                </div>
            </div>
        </section>
    }
}
