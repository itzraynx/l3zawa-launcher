/**
 * L3zawa custom skin server integration.
 *
 * Wraps the Tauri `plugin:l3zawa-skins|*` commands so the frontend can
 * upload/fetch/remove skins and capes on the L3zawa skin server.
 *
 * The skin server stores skins keyed by the active account's username, and
 * serves them in CustomSkinAPI format so the CustomSkinLoader mod (auto-
 * injected into every profile) can fetch them in-game.
 */
import { invoke } from '@tauri-apps/api/core'

export interface L3zawaSkin {
	hash: string
	variant: string
	uploadedAt: number
	url: string
}

export interface L3zawaCape {
	hash: string
	uploadedAt: number
	url: string
}

export interface L3zawaPlayerSkinInfo {
	username: string
	skin: L3zawaSkin | null
	cape: L3zawaCape | null
}

/** Upload a skin PNG for the active account. variant = 'CLASSIC' | 'SLIM'. */
export async function uploadSkin(pngBytes: Uint8Array, variant: string): Promise<L3zawaSkin> {
	return await invoke('plugin:l3zawa-skins|l3zawa_upload_skin', {
		pngBytes: Array.from(pngBytes),
		variant,
	})
}

/** Upload a cape PNG for the active account. */
export async function uploadCape(pngBytes: Uint8Array): Promise<L3zawaCape> {
	return await invoke('plugin:l3zawa-skins|l3zawa_upload_cape', {
		pngBytes: Array.from(pngBytes),
	})
}

/** Get the current skin + cape info for the active account. */
export async function getMySkinInfo(): Promise<L3zawaPlayerSkinInfo> {
	return await invoke('plugin:l3zawa-skins|l3zawa_get_my_skin_info')
}

/** Remove the active account's skin from the skin server. */
export async function removeMySkin(): Promise<void> {
	await invoke('plugin:l3zawa-skins|l3zawa_remove_my_skin')
}

/** Remove the active account's cape from the skin server. */
export async function removeMyCape(): Promise<void> {
	await invoke('plugin:l3zawa-skins|l3zawa_remove_my_cape')
}

/** Returns the full URL for a texture path on the skin server (for previews). */
export async function textureUrl(path: string): Promise<string> {
	return await invoke('plugin:l3zawa-skins|l3zawa_texture_url', { path })
}

/** Returns the configured skin server URL. */
export async function getSkinServerUrl(): Promise<string> {
	return await invoke('plugin:l3zawa-skins|l3zawa_get_skin_server_url')
}
