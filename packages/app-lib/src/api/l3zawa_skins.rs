//! L3zawa custom skin server integration.
//!
//! This module provides functions for the launcher to talk to a L3zawa skin
//! server (see `mini-services/skin-server`). The skin server stores skins and
//! capes uploaded by launcher users and serves them in CustomSkinAPI format,
//! which the CustomSkinLoader mod (auto-injected into every profile) fetches
//! so that all launcher users see each other's custom skins in-game.

use serde::{Deserialize, Serialize};
use std::time::Duration;

use crate::util::io::IOError;

const DEFAULT_SKIN_SERVER_URL: &str = "http://localhost:3001";

/// A custom skin stored on the L3zawa skin server.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct L3zawaSkin {
    pub hash: String,
    pub variant: String,
    #[serde(rename = "uploadedAt")]
    pub uploaded_at: i64,
    pub url: String,
}

/// A custom cape stored on the L3zawa skin server.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct L3zawaCape {
    pub hash: String,
    #[serde(rename = "uploadedAt")]
    pub uploaded_at: i64,
    pub url: String,
}

/// The skin + cape info for a player, as returned by the skin server.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct L3zawaPlayerSkinInfo {
    pub username: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skin: Option<L3zawaSkin>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cape: Option<L3zawaCape>,
}

/// Returns the configured skin server URL, falling back to the default.
pub async fn get_skin_server_url() -> String {
    let state = match crate::State::get().await {
        Ok(s) => s,
        Err(_) => return DEFAULT_SKIN_SERVER_URL.to_string(),
    };
    let settings = match crate::state::Settings::get(&state.pool).await {
        Ok(s) => s,
        Err(_) => return DEFAULT_SKIN_SERVER_URL.to_string(),
    };
    // Stored in custom_env_vars to avoid a schema migration. Look for a var
    // named L3ZAWA_SKIN_SERVER_URL.
    for (k, v) in &settings.custom_env_vars {
        if k == "L3ZAWA_SKIN_SERVER_URL" {
            return v.clone();
        }
    }
    DEFAULT_SKIN_SERVER_URL.to_string()
}

/// Returns the username to use for skin operations: the active account's name.
async fn get_active_username() -> crate::Result<String> {
    let state = crate::State::get().await?;
    let creds = super::Credentials::get_active(&state.pool).await?;
    match creds {
        Some(c) => Ok(c.offline_profile.name),
        None => Err(
            crate::ErrorKind::OtherError(
                "No active Minecraft account. Sign in or add an offline account first."
                    .to_string(),
            )
            .as_error()
            .into(),
        ),
    }
}

/// Uploads a skin PNG to the skin server for the active account.
///
/// `png_bytes` is the raw PNG file data. `variant` is "CLASSIC" or "SLIM".
pub async fn upload_skin(
    png_bytes: Vec<u8>,
    variant: String,
) -> crate::Result<L3zawaSkin> {
    let server_url = get_skin_server_url().await;
    let username = get_active_username().await?;

    let part = reqwest::multipart::Part::bytes(png_bytes)
        .file_name("skin.png")
        .mime_str("image/png")
        .map_err(|e| {
            crate::ErrorKind::OtherError(format!("Invalid mime: {e}")).as_error()
        })?;
    let form = reqwest::multipart::Form::new()
        .text("username", username.clone())
        .text("variant", variant)
        .part("file", part);

    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(30))
        .build()
        .map_err(|e| {
            crate::ErrorKind::OtherError(format!("HTTP client error: {e}"))
                .as_error()
        })?;

    let resp = client
        .post(format!("{server_url}/api/skin"))
        .multipart(form)
        .send()
        .await
        .map_err(|e| {
            crate::ErrorKind::OtherError(format!(
                "Failed to reach skin server at {server_url}: {e}. Make sure the L3zawa skin server is running."
            ))
            .as_error()
        })?;

    if !resp.status().is_success() {
        let status = resp.status();
        let body = resp.text().await.unwrap_or_default();
        return Err(crate::ErrorKind::OtherError(format!(
            "Skin server returned {status}: {body}"
        ))
        .as_error()
        .into());
    }

    // Parse {"ok":true,"username":...,"skin":{...}}
    #[derive(Deserialize)]
    struct UploadResp {
        skin: L3zawaSkin,
    }
    let parsed: UploadResp = resp
        .json()
        .await
        .map_err(|e| {
            crate::ErrorKind::OtherError(format!("Bad response from skin server: {e}"))
                .as_error()
        })?;
    Ok(parsed.skin)
}

/// Uploads a cape PNG to the skin server for the active account.
pub async fn upload_cape(png_bytes: Vec<u8>) -> crate::Result<L3zawaCape> {
    let server_url = get_skin_server_url().await;
    let username = get_active_username().await?;

    let part = reqwest::multipart::Part::bytes(png_bytes)
        .file_name("cape.png")
        .mime_str("image/png")
        .map_err(|e| {
            crate::ErrorKind::OtherError(format!("Invalid mime: {e}")).as_error()
        })?;
    let form = reqwest::multipart::Form::new()
        .text("username", username.clone())
        .part("file", part);

    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(30))
        .build()
        .map_err(|e| {
            crate::ErrorKind::OtherError(format!("HTTP client error: {e}"))
                .as_error()
        })?;

    let resp = client
        .post(format!("{server_url}/api/cape"))
        .multipart(form)
        .send()
        .await
        .map_err(|e| {
            crate::ErrorKind::OtherError(format!(
                "Failed to reach skin server at {server_url}: {e}"
            ))
            .as_error()
        })?;

    if !resp.status().is_success() {
        let status = resp.status();
        let body = resp.text().await.unwrap_or_default();
        return Err(crate::ErrorKind::OtherError(format!(
            "Skin server returned {status}: {body}"
        ))
        .as_error()
        .into());
    }

    #[derive(Deserialize)]
    struct UploadResp {
        cape: L3zawaCape,
    }
    let parsed: UploadResp = resp
        .json()
        .await
        .map_err(|e| {
            crate::ErrorKind::OtherError(format!("Bad response from skin server: {e}"))
                .as_error()
        })?;
    Ok(parsed.cape)
}

/// Gets the current skin + cape info for the active account from the skin server.
pub async fn get_my_skin_info() -> crate::Result<L3zawaPlayerSkinInfo> {
    let server_url = get_skin_server_url().await;
    let username = get_active_username().await?;

    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(15))
        .build()
        .map_err(|e| {
            crate::ErrorKind::OtherError(format!("HTTP client error: {e}"))
                .as_error()
        })?;

    let resp = client
        .get(format!("{server_url}/api/skin/{username}"))
        .send()
        .await
        .map_err(|e| {
            crate::ErrorKind::OtherError(format!(
                "Failed to reach skin server at {server_url}: {e}"
            ))
            .as_error()
        })?;

    if resp.status() == reqwest::StatusCode::NOT_FOUND {
        return Ok(L3zawaPlayerSkinInfo {
            username,
            skin: None,
            cape: None,
        });
    }
    if !resp.status().is_success() {
        let status = resp.status();
        let body = resp.text().await.unwrap_or_default();
        return Err(crate::ErrorKind::OtherError(format!(
            "Skin server returned {status}: {body}"
        ))
        .as_error()
        .into());
    }

    let info: L3zawaPlayerSkinInfo = resp
        .json()
        .await
        .map_err(|e| {
            crate::ErrorKind::OtherError(format!("Bad response from skin server: {e}"))
                .as_error()
        })?;
    Ok(info)
}

/// Removes the active account's skin from the skin server.
pub async fn remove_my_skin() -> crate::Result<()> {
    let server_url = get_skin_server_url().await;
    let username = get_active_username().await?;
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(15))
        .build()
        .map_err(|e| {
            crate::ErrorKind::OtherError(format!("HTTP client error: {e}"))
                .as_error()
        })?;
    let _ = client
        .delete(format!("{server_url}/api/skin/{username}"))
        .send()
        .await
        .map_err(|e| {
            crate::ErrorKind::OtherError(format!(
                "Failed to reach skin server: {e}"
            ))
            .as_error()
        })?;
    Ok(())
}

/// Removes the active account's cape from the skin server.
pub async fn remove_my_cape() -> crate::Result<()> {
    let server_url = get_skin_server_url().await;
    let username = get_active_username().await?;
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(15))
        .build()
        .map_err(|e| {
            crate::ErrorKind::OtherError(format!("HTTP client error: {e}"))
                .as_error()
        })?;
    let _ = client
        .delete(format!("{server_url}/api/cape/{username}"))
        .send()
        .await
        .map_err(|e| {
            crate::ErrorKind::OtherError(format!(
                "Failed to reach skin server: {e}"
            ))
            .as_error()
        })?;
    Ok(())
}

/// Returns the full URL for a texture on the skin server (for preview in UI).
pub async fn texture_url(path: &str) -> String {
    let server_url = get_skin_server_url().await;
    let path = path.trim_start_matches('/');
    format!("{server_url}/{path}")
}

/// The URL of the CustomSkinLoader mod jar to download and inject.
///
/// This is the official CustomSkinLoader release hosted on Modrinth. It is
/// compatible with Fabric and Forge, and supports fetching skins from any
/// CustomSkinAPI-compatible server.
const CSL_MOD_URL: &str = "https://cdn.modrinth.com/data/idYCQMpV/versions/3n5n3Gjh/CustomSkinLoader_Fabric-3.0.6.jar";

/// The local filename for the injected mod jar.
const CSL_MOD_FILENAME: &str = "l3zawa-customskinloader.jar";

/// The CSL config file that tells the mod to fetch skins from the L3zawa
/// skin server. Written to `<profile>/CustomSkinLoader/CustomSkinLoader.json`.
const CSL_CONFIG_DIR: &str = "CustomSkinLoader";
const CSL_CONFIG_FILENAME: &str = "CustomSkinLoader.json";

/// Injects the CustomSkinLoader mod + config into a profile so that skins
/// from the L3zawa skin server are fetched in-game.
///
/// This is called before launching Minecraft. It is best-effort: any error
/// is logged and the game still launches (just without cross-user skins).
///
/// The mod is only injected for Fabric/Forge/Quilt/NeoForge profiles — vanilla
/// profiles have no mod loading capability.
pub async fn inject_skin_mod(
    profile: &crate::state::Profile,
) -> crate::Result<()> {
    use crate::state::ModLoader;
    use tokio::fs;

    // Only inject for modded profiles. Vanilla can't load mods.
    match profile.loader {
        ModLoader::Fabric
        | ModLoader::Forge
        | ModLoader::NeoForge
        | ModLoader::Quilt => {}
        ModLoader::Vanilla => return Ok(()),
    }

    let profile_path = crate::api::profile::get_full_path(&profile.path).await?;
    let mods_dir = profile_path.join("mods");
    let csl_jar_path = mods_dir.join(CSL_MOD_FILENAME);

    // Ensure the mods directory exists
    fs::create_dir_all(&mods_dir).await.map_err(IOError::from)?;

    // Download the CSL jar if not already present
    if !csl_jar_path.exists() {
        tracing::info!("Downloading CustomSkinLoader mod to {}", csl_jar_path.display());
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(60))
            .build()
            .map_err(|e| {
                crate::ErrorKind::OtherError(format!("HTTP client error: {e}"))
                    .as_error()
            })?;
        let resp = client.get(CSL_MOD_URL).send().await.map_err(|e| {
            crate::ErrorKind::OtherError(format!(
                "Failed to download CustomSkinLoader mod: {e}"
            ))
            .as_error()
        })?;
        if !resp.status().is_success() {
            return Err(crate::ErrorKind::OtherError(format!(
                "CustomSkinLoader download returned {}",
                resp.status()
            ))
            .as_error()
            .into());
        }
        let bytes = resp.bytes().await.map_err(|e| {
            crate::ErrorKind::OtherError(format!("Failed to read mod bytes: {e}"))
                .as_error()
        })?;
        fs::write(&csl_jar_path, &bytes).await.map_err(IOError::from)?;
        tracing::info!("CustomSkinLoader mod installed ({} bytes)", bytes.len());
    }

    // Write the CSL config pointing to our skin server
    let server_url = get_skin_server_url().await;
    let config_dir = profile_path.join(CSL_CONFIG_DIR);
    fs::create_dir_all(&config_dir).await.map_err(IOError::from)?;
    let config_path = config_dir.join(CSL_CONFIG_FILENAME);

    // CustomSkinLoader config format:
    // {
    //   "enable": true,
    //   "forceRefreshSkin": false,
    //   "loadingType": 0,
    //   "skinSite": { "type": "CustomSkinAPI", "name": "L3zawa", "root": "http://..." }
    // }
    let config = serde_json::json!({
        "enable": true,
        "forceRefreshSkin": false,
        "loadingType": 0,
        "skinSite": {
            "type": "CustomSkinAPI",
            "name": "L3zawa",
            "root": format!("{}/", server_url.trim_end_matches('/'))
        }
    });
    let config_str = serde_json::to_string_pretty(&config).map_err(|e| {
        crate::ErrorKind::OtherError(format!("Failed to serialize CSL config: {e}"))
            .as_error()
    })?;
    fs::write(&config_path, config_str.as_bytes())
        .await
        .map_err(IOError::from)?;
    tracing::info!("CustomSkinLoader config written to {}", config_path.display());

    Ok(())
}
