use leptos::prelude::*;

#[component]
pub fn simple_counter() -> impl IntoView {
    let value = RwSignal::new(0);

    let increment = move |_| value.update(|val| *val += 1);

    let decrement = move |_| value.update(|val| *val -= 1);

    let clear = move |_| value.set(0);

    view! {
        <div>
            <h1> Simple Counter</h1>

            <button on:click = decrement> "Increment Value" </button>

            <span> "Value: "{value} </span>

            <button on:click = increment> "Decrement Value" </button>

            <button on:click=clear>"Reset"</button>
        </div>
    }
}
