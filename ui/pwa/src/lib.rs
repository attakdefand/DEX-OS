//! PWA UI for DEX-OS
//!
//! This crate provides the web-based user interface for the DEX-OS.

use yew::prelude::*;

mod components;
use components::swap::Swap;

#[function_component(App)]
fn app() -> Html {
    html! {
        <div class="app">
            <header>
                <h1>{"DEX-OS"}</h1>
            </header>
            <main>
                <Swap />
            </main>
        </div>
    }
}

#[wasm_bindgen::prelude::wasm_bindgen(start)]
pub fn run_app() {
    yew::start_app::<App>();
}
