#![allow(clippy::redundant_closure)]

mod app;
mod pages;
mod router;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
