use leptos::prelude::*;

use crate::utils::query::{QueryUtils, QUERY};

#[component]
pub fn PageQuery() -> impl IntoView {
    let demo_query = QueryUtils::extract(QUERY::DEMO.to_string());
    view! {
        <div>
            <h1>"Page Query"</h1>
            <p>"Demo Query is: " {demo_query}</p>
        </div>
    }
}
