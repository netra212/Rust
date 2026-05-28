use leptos::prelude::*;

#[component]
pub fn Header() -> impl IntoView {
    view! {
        <header>
            <nav class="flex-gap-4 bg-neutral-50">
                <a href="/">"Home"</a>
                <a href="/user">"User"</a>
                <a href="/user/123">"User"</a>
                <a href="/page-query?demo=rust-is-awesome">"Query"</a>
            </nav>
        </header>
    }
}
