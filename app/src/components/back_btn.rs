use yew::prelude::*;
use yew_icons::{Icon, IconId};
use yew_router::prelude::*;

use crate::router::Route;

#[function_component(BackBtn)]
pub fn back_btn() -> Html {
    html! {
        <Link<Route> to={Route::Home} classes="btn btn-secondary btn-soft mb-4">
            <Icon icon_id={IconId::HeroiconsOutlineArrowUturnLeft}/>
            { "Back" }
        </Link<Route>>
    }
}
