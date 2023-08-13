#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub async fn run() -> anyhow::Result<()> {
    tauri::Builder::default().run(tauri::generate_context!())?;

    Ok(())
}
