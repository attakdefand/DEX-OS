// Swap Component
// This is a placeholder for the swap functionality

use yew::prelude::*;

#[function_component(Swap)]
pub fn swap() -> Html {
    html! {
        <div class="swap-component">
            <h2>{"Swap Tokens"}</h2>
            <div class="swap-form">
                <input type="text" placeholder="From" />
                <input type="text" placeholder="To" />
                <button>{"Swap"}</button>
            </div>
        </div>
    }
}
