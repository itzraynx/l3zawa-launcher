use crate::api::Result;
use tauri::plugin::TauriPlugin;
use tauri::Runtime;
use theseus::l3zawa_skins::{
    L3zawaCape, L3zawaPlayerSkinInfo, L3zawaSkin,
    get_my_skin_info, get_skin_server_url, remove_my_cape, remove_my_skin,
    texture_url, upload_cape, upload_skin,
};
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
) -> Result<L3zawaSkin> {
    Ok(upload_skin(png_bytes, variant).await?)
}

/// Uploads a cape PNG to the L3zawa skin server for the active account.
#[tauri::command]
pub async fn l3zawa_upload_cape(png_bytes: Vec<u8>) -> Result<L3zawaCape> {
    Ok(upload_cape(png_bytes).await?)
}

/// Gets the current skin + cape info for the active account from the skin server.
#[tauri::command]
pub async fn l3zawa_get_my_skin_info() -> Result<L3zawaPlayerSkinInfo> {
    Ok(get_my_skin_info().await?)
}

/// Removes the active account's skin from the skin server.
#[tauri::command]
pub async fn l3zawa_remove_my_skin() -> Result<()> {
    Ok(remove_my_skin().await?)
}

/// Removes the active account's cape from the skin server.
#[tauri::command]
pub async fn l3zawa_remove_my_cape() -> Result<()> {
    Ok(remove_my_cape().await?)
}

/// Returns the full URL for a texture path on the skin server (for UI previews).
#[tauri::command]
pub async fn l3zawa_texture_url(path: String) -> Result<String> {
    Ok(texture_url(&path).await)
}

/// Returns the configured skin server URL.
#[tauri::command]
pub async fn l3zawa_get_skin_server_url() -> Result<String> {
    Ok(get_skin_server_url().await)
}
