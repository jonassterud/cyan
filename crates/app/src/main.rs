#![cfg_attr(all(not(debug_assertions), target_os = "windows"), windows_subsystem = "windows")]

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    #[cfg(desktop)]
    cyan_app::run().await?;

    Ok(())
}
