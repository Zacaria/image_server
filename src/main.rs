use errors::Result;
use tracing::dispatcher::set_global_default;
use tracing_subscriber::{fmt, layer::SubscriberExt, EnvFilter, Layer, Registry};

mod api;
mod errors;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();

    setup_logging();

    let router = api::app();

    listen(router).await?;

    Ok(())
}

async fn listen(router: axum::Router) -> Result<()> {
    let port = std::env::var("PORT").unwrap_or("3000".to_string());
    let addr = format!("0.0.0.0:{}", port);
    let listener = tokio::net::TcpListener::bind(addr).await?;

    tracing::info!("listening on {}", listener.local_addr()?);

    axum::serve(listener, router).await?;
    Ok(())
}

const DEFAULT_LOG_FILTER: &str = "info";
fn setup_logging() {
    let console_layer = fmt::layer().with_writer(std::io::stdout).with_filter(
        EnvFilter::try_from_default_env().unwrap_or_else(|_| DEFAULT_LOG_FILTER.into()),
    );

    let subscriber = Registry::default().with(console_layer);

    set_global_default(subscriber.into()).expect("Failed to set log subscriber");
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
