/**
 * L3zawa Skin Server
 *
 * Implements the CustomSkinAPI format used by CustomSkinLoader mod.
 * Stores skins/capes on the filesystem and serves them so that anyone
 * using the L3zawa Launcher sees custom skins in-game.
 *
 * Endpoints:
 *   CustomSkinAPI (consumed by the mod in-game):
 *     GET /<username>.json          → profile JSON { skins, capes }
 *     GET /textures/<hash>.png      → skin/cape PNG
 *
 *   Launcher API (consumed by L3zawa Launcher frontend):
 *     POST /api/skin                → upload skin (multipart: file, username, variant)
 *     POST /api/cape                → upload cape (multipart: file, username)
 *     GET  /api/skin/:username      → current skin info
 *     DELETE /api/skin/:username    → remove skin
 *     DELETE /api/cape/:username    → remove cape
 *     GET  /api/health              → health check
 *
 * Storage layout:
 *   data/users.json                 → { username: { skin: {hash, variant}, cape: {hash} } }
 *   textures/<hash>.png             → skin/cape PNG files
 */

import { Hono } from 'hono'
import { serve } from 'bun'
import { existsSync, mkdirSync, readFileSync, writeFileSync, unlinkSync } from 'node:fs'
import { join, dirname } from 'node:path'
import { fileURLToPath } from 'node:url'
import { createHash } from 'node:crypto'

const __dirname = dirname(fileURLToPath(import.meta.url))
const PORT = 3001
const DATA_DIR = join(__dirname, 'data')
const TEXTURES_DIR = join(__dirname, 'textures')
const USERS_DB = join(DATA_DIR, 'users.json')

// Ensure directories exist
mkdirSync(DATA_DIR, { recursive: true })
mkdirSync(TEXTURES_DIR, { recursive: true })

// Load or initialize the user database
type UserSkin = { hash: string; variant: 'CLASSIC' | 'SLIM'; uploadedAt: number }
type UserCape = { hash: string; uploadedAt: number }
type UserRecord = { skin?: UserSkin; cape?: UserCape }
type UsersDB = Record<string, UserRecord>

function loadDB(): UsersDB {
  if (existsSync(USERS_DB)) {
    try {
      return JSON.parse(readFileSync(USERS_DB, 'utf-8'))
    } catch {
      return {}
    }
  }
  return {}
}

function saveDB(db: UsersDB) {
  writeFileSync(USERS_DB, JSON.stringify(db, null, 2))
}

function hashBuffer(buf: Buffer): string {
  return createHash('sha256').update(buf).digest('hex').slice(0, 32)
}

function normalizeUsername(name: string): string {
  return name.trim().toLowerCase().replace(/[^a-z0-9_]/g, '')
}

const app = new Hono()

// --- Health check ---
app.get('/api/health', (c) => c.json({ ok: true, service: 'l3zawa-skin-server' }))

// --- CustomSkinAPI: profile JSON (consumed by the mod in-game) ---
// Returns skin + cape URLs in the format CustomSkinLoader expects.
// We use a wildcard match and parse the .json suffix ourselves because
// Hono's path-to-regexp doesn't reliably support the `:name.ext` syntax.
app.get('/*', (c) => {
  const path = c.req.path
  // Match /<username>.json
  const profileMatch = path.match(/^\/([^/]+)\.json$/i)
  if (profileMatch) {
    const username = normalizeUsername(decodeURIComponent(profileMatch[1]))
    const db = loadDB()
    const user = db[username]
    if (!user) {
      return c.json({}, 404)
    }
    const result: { skins: Record<string, string>; capes: Record<string, string> } = {
      skins: {},
      capes: {},
    }
    if (user.skin) {
      const key = user.skin.variant === 'SLIM' ? 'slim' : 'default'
      result.skins[key] = `textures/${user.skin.hash}.png`
    }
    if (user.cape) {
      result.capes.default = `textures/${user.cape.hash}.png`
    }
    return c.json(result)
  }

  // Match /textures/<hash>.png
  const textureMatch = path.match(/^\/textures\/([a-f0-9]+)\.png$/i)
  if (textureMatch) {
    const hash = textureMatch[1]
    const filePath = join(TEXTURES_DIR, `${hash}.png`)
    if (!existsSync(filePath)) {
      return c.notFound()
    }
    const buf = readFileSync(filePath)
    return new Response(buf, {
      headers: { 'Content-Type': 'image/png', 'Cache-Control': 'public, max-age=3600' },
    })
  }

  // Not a CustomSkinAPI route — pass through (let other handlers run)
  return c.notFound()
})

// --- Launcher API: upload skin ---
app.post('/api/skin', async (c) => {
  const formData = await c.req.formData()
  const file = formData.get('file') as File | null
  const username = normalizeUsername((formData.get('username') as string) || '')
  const variant = ((formData.get('variant') as string) || 'CLASSIC').toUpperCase() === 'SLIM' ? 'SLIM' : 'CLASSIC'

  if (!file || !username) {
    return c.json({ error: 'Missing file or username' }, 400)
  }

  if (!file.type.startsWith('image/png')) {
    return c.json({ error: 'Skin must be a PNG file' }, 400)
  }

  const buf = Buffer.from(await file.arrayBuffer())
  const hash = hashBuffer(buf)
  const skinPath = join(TEXTURES_DIR, `${hash}.png`)

  // Save texture (deduplicated by hash)
  if (!existsSync(skinPath)) {
    writeFileSync(skinPath, buf)
  }

  // Update user record
  const db = loadDB()
  if (!db[username]) db[username] = {}
  db[username].skin = { hash, variant, uploadedAt: Date.now() }
  saveDB(db)

  return c.json({
    ok: true,
    username,
    skin: { hash, variant, url: `/textures/${hash}.png` },
  })
})

// --- Launcher API: upload cape ---
app.post('/api/cape', async (c) => {
  const formData = await c.req.formData()
  const file = formData.get('file') as File | null
  const username = normalizeUsername((formData.get('username') as string) || '')

  if (!file || !username) {
    return c.json({ error: 'Missing file or username' }, 400)
  }

  if (!file.type.startsWith('image/png')) {
    return c.json({ error: 'Cape must be a PNG file' }, 400)
  }

  const buf = Buffer.from(await file.arrayBuffer())
  const hash = hashBuffer(buf)
  const capePath = join(TEXTURES_DIR, `${hash}.png`)

  if (!existsSync(capePath)) {
    writeFileSync(capePath, buf)
  }

  const db = loadDB()
  if (!db[username]) db[username] = {}
  db[username].cape = { hash, uploadedAt: Date.now() }
  saveDB(db)

  return c.json({
    ok: true,
    username,
    cape: { hash, url: `/textures/${hash}.png` },
  })
})

// --- Launcher API: get skin info ---
app.get('/api/skin/:username', (c) => {
  const username = normalizeUsername(c.req.param('username'))
  const db = loadDB()
  const user = db[username]
  if (!user) {
    return c.json({ username, skin: null, cape: null }, 404)
  }
  return c.json({
    username,
    skin: user.skin
      ? { ...user.skin, url: `/textures/${user.skin.hash}.png` }
      : null,
    cape: user.cape
      ? { ...user.cape, url: `/textures/${user.cape.hash}.png` }
      : null,
  })
})

// --- Launcher API: remove skin ---
app.delete('/api/skin/:username', (c) => {
  const username = normalizeUsername(c.req.param('username'))
  const db = loadDB()
  if (db[username]?.skin) {
    delete db[username].skin
    saveDB(db)
  }
  return c.json({ ok: true })
})

// --- Launcher API: remove cape ---
app.delete('/api/cape/:username', (c) => {
  const username = normalizeUsername(c.req.param('username'))
  const db = loadDB()
  if (db[username]?.cape) {
    delete db[username].cape
    saveDB(db)
  }
  return c.json({ ok: true })
})

// --- Root ---
app.get('/', (c) =>
  c.json({
    service: 'L3zawa Skin Server',
    version: '1.0.0',
    endpoints: {
      customSkinAPI: '/<username>.json, /textures/<hash>.png',
      launcherAPI: '/api/skin, /api/cape, /api/skin/:username',
    },
  }),
)

console.log(`🎨 L3zawa Skin Server running on http://localhost:${PORT}`)
serve({
  fetch: app.fetch,
  port: PORT,
})
