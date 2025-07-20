use gloo_storage::LocalStorage;
use gloo_storage::Storage;
use web_sys::HtmlInputElement;
use yew::html::onchange::Event;
use yew::prelude::*;

struct Theme {
    name: String,
    value: String,
}

#[function_component(ThemeController)]
pub fn theme_controller() -> Html {
    let themes = vec![
        Theme {
            name: "Default".to_string(),
            value: "default".to_string(),
        },
        Theme {
            name: "Light".to_string(),
            value: "fantasy".to_string(),
        },
        Theme {
            name: "Dark".to_string(),
            value: "dracula".to_string(),
        },
    ];

    // LocalStorage get can trigger multiple times so need to use `use_state_eq` to avoid infinite loop
    let selected_theme = use_state_eq(|| "default".to_string());
    LocalStorage::get::<String>("theme").ok().inspect(|t| {
        if themes.iter().any(|theme| &theme.value == t) {
            selected_theme.set(t.clone());
        }
    });

    let on_change = {
        let selected_theme = selected_theme.clone();
        Callback::from(move |e: Event| {
            let selected_theme = selected_theme.clone();
            e.prevent_default();
            let input: HtmlInputElement = e.target_unchecked_into();
            let value = input.value();
            selected_theme.set(value.clone());
            LocalStorage::set("theme", value).unwrap();
        })
    };

    html! {
        <div class="dropdown dropdown-end">
            <div tabindex="0" role="button" class="btn m-1">
                { "Theme" }
                <svg
                    width="12px"
                    height="12px"
                    class="inline-block h-2 w-2 fill-current opacity-60"
                    xmlns="http://www.w3.org/2000/svg"
                    viewBox="0 0 2048 2048"
                >
                <path d="M1799 349l242 241-1017 1017L7 590l242-241 775 775 775-775z"></path>
                </svg>
            </div>
            <ul tabindex="0" class="dropdown-content bg-base-300 rounded-box z-1 w-30 p-2 shadow-2xl">
                {
                    themes.into_iter().map(|theme| {
                        html! {
                            <li>
                                <input
                                    type="radio"
                                    name="theme-dropdown"
                                    class="theme-controller w-full btn btn-sm btn-block btn-ghost justify-start"
                                    aria-label={theme.name}
                                    value={theme.value.clone()}
                                    onchange={on_change.clone()}
                                    checked={*selected_theme == theme.value}
                                />
                            </li>
                        }
                    }).collect::<Html>()
                }
            </ul>
        </div>
    }
}
