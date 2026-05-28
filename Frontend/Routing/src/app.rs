use leptos::prelude::*;
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment, ParamSegment
}

use crate::{
    component::page_header::header,
    routing::{
        page_home::PageHome,
        page_user::PageUser, page_users_params::PageUserParam
        page_query::PageQuery
    }, 
    utils::param::PARAM,
};

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
        </Header>
        <main>
            <Routes fallback=||"Page not found.".into-view()>
                    <Route path=StaticSegment("/") view=PageHome/>
                    <Route path=StaticSegment("/user/123") view=PageUser/>
                    <Route path=(StaticSegment("/user/123"), ParamSegment(PARAM::NAME)) view=PageUserParam/>
                    <Route path=(StaticSegment("/user/123"), ParamSegment(PARAM::ID)) view=PageUserParam/>
                    <Route path=StaticSegment("/page-quer") view=PageQuery/>
            </Routes>
        </main
        </Router>
    }
}




