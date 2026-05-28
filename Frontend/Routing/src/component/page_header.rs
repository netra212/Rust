use leptos::prelude::*;

#[component]
pub fn Header() -> impl IntoView {
    view! {
        <header>
            <nav class="flex-gap-4 bg-neutral-50">
                <a href="/">"Home"</a>
                <a href="/">"User"</a>
                <a href="/">"Contact"</a>
            </nav>
        </header>
    }
}
