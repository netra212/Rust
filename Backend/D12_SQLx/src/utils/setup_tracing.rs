use tracing_subscriber::layer::SubscriberExt;
// Imports the `SubscriberExt` trait.
// Provides the `.with()` method used to attach tracing layers.

use tracing_subscriber::util::SubscriberInitExt;
// Imports the `SubscriberInitExt` trait.
// Provides the `.init()` method to initialize the tracing subscriber.

pub fn setup_tracing() {
    // Public function used to configure application logging/tracing.

    tracing_subscriber::registry()
        // Creates a new tracing subscriber registry.
        // This acts as the base collector for all tracing data.
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                // Attempts to read log filters from the `RUST_LOG` environment variable.
                // Example:
                // RUST_LOG=debug
                // RUST_LOG=my_app=trace,tower_http=info

                format!("{}=debug,tower_http=debug", env!("CARGO_CRATE_NAME")).into()
                // If `RUST_LOG` is not set,
                // use a default filter configuration:
                //
                // - Current crate logs at DEBUG level
                // - tower_http logs at DEBUG level
                //
                // `env!("CARGO_CRATE_NAME")`
                // gets the project name from Cargo at compile time.
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        // Adds a formatting layer.
        // This prints logs nicely to the terminal/stdout.
        .init();
    // Initializes the global tracing subscriber.
    // After this, all `tracing::info!`, `debug!`, etc. macros work.
}
