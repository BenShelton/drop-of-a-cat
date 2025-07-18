use yew::prelude::*;
use yew_router::prelude::*;

use crate::{
    components::navbar::Navbar,
    router::{switch, Route},
};

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            // Needs to be within the BrowserRouter
            // otherwise the link to Home will not work
            <Navbar />
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}
