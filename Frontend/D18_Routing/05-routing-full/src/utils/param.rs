use leptos::prelude::*;
use leptos_router::{hooks::use_params, params::Params};

#[derive(Params, PartialEq, Clone)]
pub struct ParamsUtils {
    pub name: Option<String>,
    pub id: Option<String>,
}

impl ParamsUtils {
    pub fn extract(param_key: String) -> Memo<Option<String>> {
        let params = use_params::<Self>();

        Memo::new(move |_| {
            let params_read = params.read();
            if let Ok(params) = params_read.as_ref() {
                match param_key.as_str() {
                    PARAM::NAME => params.name.clone(),
                    PARAM::ID => params.id.clone(),
                    _ => None,
                }
            } else {
                None
            }
        })
    }
}

pub struct PARAM {}

impl PARAM {
    // TODO: Set the correct value for NAME constant
    // Hint: It should be "name"
    pub const NAME: &str = "name";

    // TODO: Set the correct value for ID constant
    // Hint: It should be "id"
    pub const ID: &str = "id";
}
