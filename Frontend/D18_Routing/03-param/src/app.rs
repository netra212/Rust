use leptos::prelude::*;
use leptos_router::{
    components::{Route, Router, Routes},
    ParamSegment, StaticSegment,
};

use crate::{
    components::header::Header,
    routing::{page_home::PageHome, page_param::PageParam},
    utils::param::PARAM,
};

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
            <Header />

            <main class="min-h-screen">
                <Routes fallback=|| "Page not found.".into_view()>
                    <Route path=StaticSegment("/") view=PageHome />

                    <Route path=(StaticSegment("/page-param"), ParamSegment(PARAM::NAME)) view=PageParam />
                </Routes>
            </main>
        </Router>
    }
}
