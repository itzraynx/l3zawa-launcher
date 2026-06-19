use crate::api::Result;
use tauri::plugin::TauriPlugin;
use tauri::Runtime;
use theseus::prelude::*;

pub fn init<R: Runtime>() -> TauriPlugin<R> {
    tauri::plugin::Builder::<R>::new("l3zawa-skins")
        .invoke_handler(tauri::generate_handler![
            l3zawa_upload_skin,
            l3zawa_upload_cape,
            l3zawa_get_my_skin_info,
            l3zawa_remove_my_skin,
            l3zawa_remove_my_cape,
            l3zawa_texture_url,
            l3zawa_get_skin_server_url,
        ])
        .build()
}

/// Uploads a skin PNG to the L3zawa skin server for the active account.
#[tauri::command]
pub async fn l3zawa_upload_skin(
    png_bytes: Vec<u8>,
    variant: String,
) -> Result<theseus::api::l3zawa_skins::L3zawaSkin> {
    Ok(theseus::api::l3zawa_skins::upload_skin(png_bytes, variant).await?)
}

/// Uploads a cape PNG to the L3zawa skin server for the active account.
#[tauri::command]
pub async fn l3zawa_upload_cape(
    png_bytes: Vec<u8>,
) -> Result<theseus::api::l3zawa_skins::L3zawaCape> {
    Ok(theseus::api::l3zawa_skins::upload_cape(png_bytes).await?)
}

/// Gets the current skin + cape info for the active account from the skin server.
#[tauri::command]
pub async fn l3zawa_get_my_skin_info(
) -> Result<theseus::api::l3zawa_skins::L3zawaPlayerSkinInfo> {
    Ok(theseus::api::l3zawa_skins::get_my_skin_info().await?)
}

/// Removes the active account's skin from the skin server.
#[tauri::command]
pub async fn l3zawa_remove_my_skin() -> Result<()> {
    Ok(theseus::api::l3zawa_skins::remove_my_skin().await?)
}

/// Removes the active account's cape from the skin server.
#[tauri::command]
pub async fn l3zawa_remove_my_cape() -> Result<()> {
    Ok(theseus::api::l3zawa_skins::remove_my_cape().await?)
}

/// Returns the full URL for a texture path on the skin server (for UI previews).
#[tauri::command]
pub async fn l3zawa_texture_url(path: String) -> Result<String> {
    Ok(theseus::api::l3zawa_skins::texture_url(&path).await)
}

/// Returns the configured skin server URL.
#[tauri::command]
pub async fn l3zawa_get_skin_server_url() -> Result<String> {
    Ok(theseus::api::l3zawa_skins::get_skin_server_url().await)
}
