# L3zawa Skin Server

A lightweight skin/cape server for the L3zawa Launcher. Stores skins uploaded
by launcher users and serves them in [CustomSkinAPI](https://github.com/flowerinsight/CustomSkinLoader) format so the
CustomSkinLoader mod (auto-injected into every profile) can fetch them in-game.

This is what makes **all L3zawa Launcher users see each other's custom skins** —
the same approach used by Feather Client, FastClient, and similar PVP launchers.

## How it works

```
┌─────────────────┐     upload skin      ┌──────────────────┐
│  L3zawa Launcher│ ───────────────────► │  L3zawa Skin     │
│  (Vue UI)       │                      │  Server (this)   │
└─────────────────┘                      │                  │
                                         │  stores skins/   │
┌─────────────────┐   fetch <user>.json  │  capes on disk   │
│  Minecraft      │ ◄──────────────────► │                  │
│  + CSL mod      │   fetch textures     └──────────────────┘
│  (in-game)      │
└─────────────────┘
```

When a L3zawa Launcher user joins a server, the CustomSkinLoader mod intercepts
Minecraft's skin loading and fetches every player's skin from this server
instead of Mojang's. So anyone using the launcher sees the custom skins.

## Run locally (for development)

```bash
cd mini-services/skin-server
bun install
bun run dev
# Server runs on http://localhost:3001
```

## Deploy to a public host (REQUIRED for cross-user skins)

For all launcher users to see each other's skins, this server must be reachable
from the internet. Pick one of these free options:

### Option 1: Render.com (free tier, easiest)

1. Fork the `l3zawa-launcher` repo to your GitHub
2. Go to https://render.com → New → Web Service → connect your repo
3. Settings:
   - **Root Directory**: `mini-services/skin-server`
   - **Build Command**: `bun install`
   - **Start Command**: `bun run start`
   - **Environment**: `Bun`
4. Deploy. You'll get a URL like `https://l3zawa-skin-server.onrender.com`
5. In the launcher, set the skin server URL to that (see below)

### Option 2: fly.io (free tier)

```bash
cd mini-services/skin-server
flyctl launch --no-deploy
# Edit fly.toml to set internal_port = 3001
flyctl deploy
```

### Option 3: Any VPS with Docker

```dockerfile
FROM oven/bun:1
WORKDIR /app
COPY package.json ./
RUN bun install
COPY . .
EXPOSE 3001
CMD ["bun", "run", "start"]
```

```bash
docker build -t l3zawa-skin-server .
docker run -p 80:3001 -v $(pwd)/data:/app/data -v $(pwd)/textures:/app/textures l3zawa-skin-server
```

## Point the launcher at your deployed server

Once your server is running at a public URL (e.g. `https://l3zawa-skins.example.com`),
configure the launcher to use it. The skin server URL is read from the
`L3ZAWA_SKIN_SERVER_URL` custom env var in the launcher settings.

To set it, add an entry to the launcher's custom env vars (Settings →
Advanced → Custom Environment Variables):

```
L3ZAWA_SKIN_SERVER_URL = https://l3zawa-skins.example.com
```

Then restart the launcher. The Skins page will show "Server online" and uploads
will go to your server.

## API

### CustomSkinAPI (consumed by the mod in-game)

| Method | Path | Description |
|---|---|---|
| `GET` | `/<username>.json` | Profile JSON: `{ skins: { default/slim: "textures/<hash>.png" }, capes: { default: "textures/<hash>.png" } }` |
| `GET` | `/textures/<hash>.png` | Skin/cape PNG |

### Launcher API (consumed by the L3zawa Launcher frontend)

| Method | Path | Description |
|---|---|---|
| `GET` | `/api/health` | Health check |
| `POST` | `/api/skin` | Upload skin (multipart: `file`, `username`, `variant`) |
| `POST` | `/api/cape` | Upload cape (multipart: `file`, `username`) |
| `GET` | `/api/skin/:username` | Current skin + cape info |
| `DELETE` | `/api/skin/:username` | Remove skin |
| `DELETE` | `/api/cape/:username` | Remove cape |

## Storage

- `data/users.json` — JSON map of username → { skin, cape }
- `textures/<sha256-prefix>.png` — deduplicated skin/cape PNG files

No database required. Back up the `data/` and `textures/` directories.

## License

GPL-3.0 (inherited from the L3zawa Launcher / Modrinth App fork)
