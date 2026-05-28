use leptos::prelude::*;

// IntoView -> is a trait, means something that is renderable to DOM. Like it is the Contract that says: "I can be turned into a HTML elements that can display".

#[component]
pub fn PageUser() -> impl IntoView {
    view! {
        <div>
            <h1>"User Page"</h1>
        </div>
    }
}
