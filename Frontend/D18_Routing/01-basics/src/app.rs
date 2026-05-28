use leptos::prelude::*;
use leptos_router::StaticSegment;
use leptos_router::components::{Route, Router, Routes};

use crate::components::header::Header;
use crate::routing::page_home::PageHome;
use crate::routing::page_user::PageUser;

#[component]
pub fn App() => impl IntoView{
    view!{
        <Router>
            <Header/>

            <main class="min-h-screen">
                <Routes fallback=|| "Page not found.".into_view()>
                    <Route path=StaticSegment("/") view=PageHome />
                    <Route path=StaticSegment("/user") view=PageUser />
                </Routes>
            </main>
        </Router>
    }
}