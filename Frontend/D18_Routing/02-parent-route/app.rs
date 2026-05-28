use leptos::prelude::*;
use leptos_router::{
    components::{ParentRoute, Route, Router, Routes},
    StaticSegment,
};

use crate::{
    components::header::Header,
    domain::user::routing::{user_layout::UserLayout, user_profile_page::UserProfilePage},
    routing::page_home::PageHome,
};

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
            <Header />

            <main class="min-h-screen">
                <Routes fallback=|| "Page not found.".into_view()>
                    <Route path=StaticSegment("/") view=PageHome />

                    // ------- PARENT ROUTE ---------
                    <ParentRoute path=StaticSegment("users") view=UserLayout>
                        <Route path=StaticSegment("") view=|| view! { <p>"Select a user to view profile"</p> } />
                        <Route path=StaticSegment("profile") view=UserProfilePage />
                    </ParentRoute>
                </Routes>
            </main>
        </Router>
    }
}
