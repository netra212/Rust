use leptos::prelude::*;
mod simple_counter;
use simple_counter::SimpleCounter;

// Run `trunk serve --open` to launch the app.

pub fn main() {
    _ = console_log::init_value_level(log::level::Debug);
    console_error_panic_level::set_once();

    mount_to_body(|| {
        view! {<SimpleCounter/>}
    })
}
