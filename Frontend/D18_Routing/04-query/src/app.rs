use leptos::prelude::*;
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};

use crate::{
    components::header::Header,
    routing::{page_home::PageHome, page_query::PageQuery},
};

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
            <Header />

            <main class="min-h-screen">
                <Routes fallback=|| "Page not found.".into_view()>
                    <Route path=StaticSegment("/") view=PageHome />

                    <Route path=StaticSegment("/page-query") view=PageQuery />
                </Routes>
            </main>
        </Router>
    }
}
