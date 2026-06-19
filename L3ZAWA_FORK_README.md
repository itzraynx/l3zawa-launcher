# L3zawa Launcher (Modrinth App Fork)

A fork of the [Modrinth App](https://github.com/modrinth/code) (codename
"Theseus") — the open-source Minecraft launcher — with two additions:

1. **Offline account login** — play Minecraft with a custom username, no
   Microsoft account required. Works on offline-mode servers and singleplayer.
2. **Custom branding** — rebranded to "L3zawa Launcher" with a custom orange
   theme color and new logo.

## What changed

### Offline login

| File | Change |
|------|--------|
| `Cargo.toml` (workspace) | Added `md-5 = "0.10.6"` dependency |
| `packages/app-lib/Cargo.toml` | Added `md-5 = { workspace = true }` |
| `packages/app-lib/src/state/minecraft_auth.rs` | Added `OFFLINE_TOKEN_MARKER` const, `offline_player_uuid()` (canonical `OfflinePlayer:<name>` MD5 type-3 UUID), `login_offline()` function, `Credentials::is_offline()` method, and guards in `refresh()` + `online_profile_with_cache_intent()` so offline accounts never contact Mojang |
| `packages/app-lib/src/api/minecraft_auth.rs` | Exposed `login_offline()` |
| `apps/app/src/api/auth.rs` | Added `login_offline` Tauri command + registered in plugin handler |
| `apps/app-frontend/src/helpers/auth.js` | Added `login_offline()` JS wrapper |
| `apps/app-frontend/src/components/ui/AddOfflineAccountModal.vue` | **New file** — the "Add new offline account" dialog (matches the design mockup: title, player-name input, green/orange Login button, X close, random-username generator button) |
| `apps/app-frontend/src/components/ui/AccountsCard.vue` | Added "Add offline account" button (shown in both the signed-out state and the account list accordion) + modal wiring |
| `apps/app-frontend/src/locales/en-US/index.json` | Added all `offline-account.*` and `minecraft-account.add-offline-account` i18n strings |

#### How offline accounts work

- The offline player UUID is computed as
  `MD5("OfflinePlayer:" + username)` with version 3 / IETF variant bits set —
  the exact same algorithm Java's `UUID.nameUUIDFromBytes` uses. This means
  the UUID is **stable across launches** and **matches what offline-mode
  servers expect** for whitelisting.
- The `access_token` is set to the sentinel string `"offline"`. The launcher
  checks for this via `Credentials::is_offline()` and short-circuits all
  Mojang API calls (token refresh, profile fetch), so offline accounts never
  trigger network requests or auth errors.
- Offline accounts are stored in the same `minecraft_users` SQLite table as
  real accounts — no schema migration needed. They appear in the account
  switcher like any other account and can be removed the same way.

### Custom branding

| File | Change |
|------|--------|
| `apps/app/tauri.conf.json` | `productName` → `"L3zawa Launcher"`, `mainBinaryName` → `"L3zawa Launcher"`, window `title` → `"L3zawa Launcher"`, `identifier` → `"com.l3zawa.launcher"` |
| `apps/app-frontend/src/assets/stylesheets/global.scss` | Added `--color-brand` override (L3zawa Orange `#ff6b35`) at the top of the file for both light and dark themes |
| `apps/app-frontend/src/assets/modrinth_app.svg` | Replaced Modrinth logo with a L3zawa star-burst SVG (original backed up as `.bak`) |
| `apps/app/icons/l3zawa-source/l3zawa-icon-1024.png` | AI-generated 1024×1024 source icon for regenerating platform icons |

#### To fully rebrand the app icons

The OS-level icons (`.ico` for Windows, `.icns` for macOS, `.png` at various
sizes) in `apps/app/icons/` are still the Modrinth ones. To regenerate them
from the Nova source image:

```bash
# From the repo root, after installing deps (pnpm install)
pnpm tauri icon apps/app/icons/l3zawa-source/l3zawa-icon-1024.png
```

This generates all required icon formats for all platforms.

#### To change the brand color

Edit the hex values in
`apps/app-frontend/src/assets/stylesheets/global.scss` (lines 19–31):

```scss
:root {
    --color-brand: #ff6b35;                    /* ← change this */
    --color-brand-highlight: rgba(255, 107, 53, 0.25);
    --color-brand-shadow: rgba(255, 107, 53, 0.55);
}
```

The SVG logo uses `var(--color-brand)`, so it automatically follows.

---

## Building with GitHub Actions (free)

This fork ships with the original Modrinth CI workflows. To build installers
for Windows, macOS, and Linux for free:

### 1. Fork to your GitHub

```bash
cd modrinth-fork
git remote add origin https://github.com/code/code.git
git push -u origin main
```

### 2. Enable Actions

On GitHub, go to your fork → **Settings** → **Actions** → **General** →
ensure "Allow all actions and reusable workflows" is selected.

### 3. Trigger a build

The workflow `.github/workflows/theseus-build.yml` runs automatically on:
- Pushes to `main` (if files under `apps/app/**`, `apps/app-frontend/**`,
  `packages/app-lib/**`, etc. changed)
- Tag pushes matching `v*` (e.g. `v0.1.0`)

To trigger a build manually: go to **Actions** tab → **Modrinth App build** →
**Run workflow** → select `main` → **Run workflow**.

### 4. Download the installers

Once the build completes (takes ~15–40 minutes depending on runners):
- Go to the workflow run → scroll to **Artifacts** at the bottom
- Download the artifact for your platform:
  - `nova-launcher-windows` → `.exe` (NSIS installer)
  - `nova-launcher-macos` → `.dmg`
  - `nova-launcher-linux` → `.AppImage` / `.deb`

> **Note:** Binaries are **unsigned** (no code-signing certificate). Windows
> SmartScreen and macOS Gatekeeper will warn on first launch — click
> "More info" → "Run anyway" (Windows) or right-click → Open (macOS).

### 5. For releases

Tag a version to trigger `theseus-release.yml`:

```bash
git tag v0.1.0
git push origin v0.1.0
```

This creates a GitHub Release with the installers attached.

---

## Local development

> Requires: Node.js 24+, pnpm 10+, Rust 1.90+ (see `rust-toolchain.toml`)

```bash
pnpm install
pnpm app:dev    # launches the Tauri dev window
```

To type-check the frontend without building:
```bash
cd apps/app-frontend && pnpm tsc:check
```

To check Rust compilation:
```bash
cargo check -p theseus
cargo check -p modrinth-app
```

---

## License

This fork inherits the Modrinth license. See `COPYING.md` and the `LICENSE`
file in each package. The Modrinth App (Theseus) is licensed under the GPL-3.0.
