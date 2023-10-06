use commons::{configuration::settings::Settings, tracing::init_tracing};
use rest::server::Server;
use tokio::join;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    init_tracing();
    let settings = Settings::load()?;
    let server = Server::from_settings(&settings).await?;
    let server_task = tokio::task::spawn(server.run());
    join!(server_task).0.unwrap();
    Ok(())
}
