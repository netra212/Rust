use leptos::prelude::*;

use crate::app::App;

/**
Part	                    Purpose	                                            React Comparison
shell	                    The function name - "shell"                         index.html file
                            because it's the outer shell
                            of your app
options: LeptosOptions	    Configuration for how Leptos                        env variables or config file
                            should run (SSR mode, hydrate mode, etc.)

impl IntoView	            Returns HTML that can be rendered                   The HTML file itself

Why is it called "shell"? Because it's the empty container that holds your app. Like an egg shell - the outside structure that contains everything inside.
 */
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
