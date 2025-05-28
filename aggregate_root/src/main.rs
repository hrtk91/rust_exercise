use aggregate_root::adapter::{cli, repository};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    sqlx::migrate!().run(&*repository::POOL).await?;
    cli::run().await
}