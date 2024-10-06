use std::env;

/// # Panics
/// Will panic if `SENTRY_DSN` is not set in the .env file
#[inline]
pub fn load_sentry() {
    dotenvy::dotenv().unwrap_or_else(|_| {
        panic!("Missing .env file");
    });

    let sentry_dns = env::var("SENTRY_DSN").unwrap_or_else(|_| {
        panic!("Missing SENTRY_DSN environment variable");
    });

    let _guard = sentry::init((
        sentry_dns,
        sentry::ClientOptions {
            release: sentry::release_name!(),
            ..Default::default()
        },
    ));
}
