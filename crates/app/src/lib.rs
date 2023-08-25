mod state;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub async fn run() -> anyhow::Result<()> {
    let state = state::AppState::new();
    state.inner()?.client.add_relay("relay.damus.io".to_string());
    state.inner()?.client.connect_relays()?;

    tauri::Builder::default().run(tauri::generate_context!())?;

    Ok(())
}
