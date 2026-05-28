use leptos::prelude::*;

use crate::routing::{ParamsUtils, PARAM};

#[component]
pub fn PageUserParam() -> impl IntoView {
    let params_users = ParamsUtils::extract(PARAM::NAME.to_string());
    let params_users_id = ParamsUtils::extract(PARAM::ID.to_string());

    view! {
        <div>
            <h1>"Page Params"</h1>
            <p>"User with Params: "{params_users}</p
        </div>
    }
}
