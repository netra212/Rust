use leptos::prelude::*;

use crate::app::App;

// this file is similar to index.html in react.
pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1" />

                <AutoReload options=options.clone() />
                <HydrationScripts options />
            </head>

            <body>
                <App />
            </body>
        </html>
    }
}
