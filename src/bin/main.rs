use loco_rs::cli;
use migration::Migrator;
use todo_app::app::App;

#[tokio::main]
async fn main() -> loco_rs::Result<()> {
    cli::main::<App, Migrator>().await
}
