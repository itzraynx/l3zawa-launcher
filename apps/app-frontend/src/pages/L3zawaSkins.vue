<template>
    <div class="l3zawa-skins-page">
        <!-- Header -->
        <div class="header-section">
            <div class="flex items-center gap-4">
                <div class="brand-logo">
                    <SkinIcon class="w-8 h-8 text-brand" />
                </div>
                <div>
                    <h1 class="text-2xl font-bold text-contrast m-0">
                        {{ formatMessage(messages.title) }}
                    </h1>
                    <p class="text-secondary text-sm m-0 mt-1">
                        {{ formatMessage(messages.subtitle) }}
                    </p>
                </div>
            </div>
            <Badge :color="serverOnline ? 'green' : 'red'" type="outlined">
                {{ serverOnline ? formatMessage(messages.serverOnline) : formatMessage(messages.serverOffline) }}
            </Badge>
        </div>

        <!-- No account warning -->
        <div v-if="!hasAccount" class="no-account-card">
            <UserRoundIcon class="w-6 h-6 text-secondary" />
            <div class="flex-1">
                <p class="font-semibold text-primary m-0">{{ formatMessage(messages.noAccountTitle) }}</p>
                <p class="text-secondary text-sm m-0">{{ formatMessage(messages.noAccountDesc) }}</p>
            </div>
        </div>

        <template v-else>
            <!-- 3D Preview -->
            <div class="preview-card">
                <SkinPreviewRenderer
                    v-if="currentSkinUrl"
                    :texture-src="currentSkinUrl"
                    :cape-src="currentCapeUrl || undefined"
                    :variant="currentVariant"
                    :nametag="username || undefined"
                    :initial-rotation="20"
                    class="preview-renderer"
                />
                <div v-else class="preview-placeholder">
                    <ChangeSkinIcon class="w-16 h-16 text-secondary" />
                    <p class="text-secondary m-0 mt-2">{{ formatMessage(messages.noSkinYet) }}</p>
                </div>
                <div v-if="username" class="nametag-overlay">
                    <span class="text-lg font-bold text-contrast">{{ username }}</span>
                </div>
            </div>

            <!-- Skin upload section -->
            <div class="upload-section">
                <h2 class="section-title">{{ formatMessage(messages.skinSection) }}</h2>
                <div class="upload-card">
                    <div class="upload-controls">
                        <label class="variant-label">{{ formatMessage(messages.model) }}</label>
                        <div class="variant-buttons">
                            <ButtonStyled :type="variant === 'CLASSIC' ? 'filled' : 'outlined'" color="brand">
                                <button @click="variant = 'CLASSIC'">
                                    {{ formatMessage(messages.classic) }}
                                </button>
                            </ButtonStyled>
                            <ButtonStyled :type="variant === 'SLIM' ? 'filled' : 'outlined'" color="brand">
                                <button @click="variant = 'SLIM'">
                                    {{ formatMessage(messages.slim) }}
                                </button>
                            </ButtonStyled>
                        </div>

                        <input
                            ref="skinFileInput"
                            type="file"
                            accept="image/png"
                            class="hidden-file-input"
                            @change="onSkinFileSelected"
                        />

                        <div class="action-buttons">
                            <ButtonStyled color="brand">
                                <button :disabled="uploading" @click="triggerSkinUpload">
                                    <UploadIcon v-if="!uploading" />
                                    <SpinnerIcon v-else class="animate-spin" />
                                    {{ formatMessage(messages.uploadSkin) }}
                                </button>
                            </ButtonStyled>
                            <ButtonStyled v-if="currentSkin" type="outlined" color="red">
                                <button :disabled="uploading" @click="removeSkin">
                                    <TrashIcon />
                                    {{ formatMessage(messages.remove) }}
                                </button>
                            </ButtonStyled>
                        </div>
                    </div>

                    <div v-if="currentSkin" class="current-info">
                        <img :src="currentSkinUrl" class="skin-thumbnail" :alt="'Current skin'" />
                        <div>
                            <p class="m-0 font-semibold text-primary">{{ formatMessage(messages.currentSkin) }}</p>
                            <p class="m-0 text-xs text-secondary">
                                {{ currentSkin.variant }} • {{ formatMessage(messages.uploadedOn) }}
                                {{ new Date(currentSkin.uploadedAt).toLocaleString() }}
                            </p>
                        </div>
                    </div>
                </div>
            </div>

            <!-- Cape upload section -->
            <div class="upload-section">
                <h2 class="section-title">{{ formatMessage(messages.capeSection) }}</h2>
                <div class="upload-card">
                    <input
                        ref="capeFileInput"
                        type="file"
                        accept="image/png"
                        class="hidden-file-input"
                        @change="onCapeFileSelected"
                    />
                    <div class="action-buttons">
                        <ButtonStyled color="brand">
                            <button :disabled="uploadingCape" @click="triggerCapeUpload">
                                <UploadIcon v-if="!uploadingCape" />
                                <SpinnerIcon v-else class="animate-spin" />
                                {{ formatMessage(messages.uploadCape) }}
                            </button>
                        </ButtonStyled>
                        <ButtonStyled v-if="currentCape" type="outlined" color="red">
                            <button :disabled="uploadingCape" @click="removeCape">
                                <TrashIcon />
                                {{ formatMessage(messages.remove) }}
                            </button>
                        </ButtonStyled>
                    </div>

                    <div v-if="currentCape" class="current-info">
                        <img :src="currentCapeUrl" class="cape-thumbnail" :alt="'Current cape'" />
                        <div>
                            <p class="m-0 font-semibold text-primary">{{ formatMessage(messages.currentCape) }}</p>
                            <p class="m-0 text-xs text-secondary">
                                {{ formatMessage(messages.uploadedOn) }}
                                {{ new Date(currentCape.uploadedAt).toLocaleString() }}
                            </p>
                        </div>
                    </div>
                </div>
            </div>

            <!-- How it works -->
            <div class="how-it-works-card">
                <h3 class="m-0 mb-2 font-bold text-primary">{{ formatMessage(messages.howItWorksTitle) }}</h3>
                <ol class="m-0 pl-4 text-sm text-secondary space-y-1">
                    <li>{{ formatMessage(messages.howStep1) }}</li>
                    <li>{{ formatMessage(messages.howStep2) }}</li>
                    <li>{{ formatMessage(messages.howStep3) }}</li>
                    <li>{{ formatMessage(messages.howStep4) }}</li>
                </ol>
            </div>
        </template>
    </div>
</template>

<script setup lang="ts">
import {
    ChangeSkinIcon,
    SkinIcon,
    SpinnerIcon,
    TrashIcon,
    UploadIcon,
    UserRoundIcon,
} from '@modrinth/assets'
import {
    Badge,
    ButtonStyled,
    defineMessages,
    SkinPreviewRenderer,
    useVIntl,
} from '@modrinth/ui'
import { computed, onMounted, ref } from 'vue'
import { useRouter } from 'vue-router'

import {
    getMySkinInfo,
    getSkinServerUrl,
    removeMyCape,
    removeMySkin,
    textureUrl,
    uploadCape,
    uploadSkin,
    type L3zawaCape,
    type L3zawaSkin,
} from '@/helpers/l3zawa_skins'
import { get_default_user, users } from '@/helpers/auth'
import { handleSevereError } from '@/store/error.js'

const { formatMessage } = useVIntl()
const router = useRouter()

const hasAccount = ref(false)
const username = ref('')
const currentSkin = ref<L3zawaSkin | null>(null)
const currentCape = ref<L3zawaCape | null>(null)
const variant = ref<'CLASSIC' | 'SLIM'>('CLASSIC')
const uploading = ref(false)
const uploadingCape = ref(false)
const serverOnline = ref(false)
const skinServerUrl = ref('')

const skinFileInput = ref<HTMLInputElement | null>(null)
const capeFileInput = ref<HTMLInputElement | null>(null)

const currentSkinUrl = computed(() => {
    if (!currentSkin.value) return ''
    // The skin server URL + the texture path
    return `${skinServerUrl.value}/${currentSkin.value.url.replace(/^\//, '')}`
})

const currentCapeUrl = computed(() => {
    if (!currentCape.value) return ''
    return `${skinServerUrl.value}/${currentCape.value.url.replace(/^\//, '')}`
})

const currentVariant = computed<'SLIM' | 'CLASSIC'>(() =>
    currentSkin.value?.variant === 'SLIM' ? 'SLIM' : 'CLASSIC',
)

async function refresh() {
    skinServerUrl.value = await getSkinServerUrl().catch(() => 'http://localhost:3001')

    // Check server health
    try {
        const resp = await fetch(`${skinServerUrl.value}/api/health`)
        serverOnline.value = resp.ok
    } catch {
        serverOnline.value = false
    }

    // Check account
    const defaultUser = await get_default_user().catch(() => null)
    if (!defaultUser) {
        hasAccount.value = false
        return
    }
    const allUsers = await users().catch(() => [])
    const active = allUsers.find((u: { profile: { id: string } }) => u.profile.id === defaultUser)
    if (!active) {
        hasAccount.value = false
        return
    }
    hasAccount.value = true
    username.value = active.profile.name

    // Fetch current skin info
    try {
        const info = await getMySkinInfo()
        currentSkin.value = info.skin
        currentCape.value = info.cape
        if (info.skin) {
            variant.value = info.skin.variant === 'SLIM' ? 'SLIM' : 'CLASSIC'
        }
    } catch {
        // Non-fatal: user may not have a skin yet
    }
}

function triggerSkinUpload() {
    skinFileInput.value?.click()
}

function triggerCapeUpload() {
    capeFileInput.value?.click()
}

async function onSkinFileSelected(e: Event) {
    const input = e.target as HTMLInputElement
    const file = input.files?.[0]
    if (!file) return

    uploading.value = true
    try {
        const bytes = new Uint8Array(await file.arrayBuffer())
        const skin = await uploadSkin(bytes, variant.value)
        currentSkin.value = skin
    } catch (err) {
        await handleSevereError(err)
    } finally {
        uploading.value = false
        input.value = ''
    }
}

async function onCapeFileSelected(e: Event) {
    const input = e.target as HTMLInputElement
    const file = input.files?.[0]
    if (!file) return

    uploadingCape.value = true
    try {
        const bytes = new Uint8Array(await file.arrayBuffer())
        const cape = await uploadCape(bytes)
        currentCape.value = cape
    } catch (err) {
        await handleSevereError(err)
    } finally {
        uploadingCape.value = false
        input.value = ''
    }
}

async function removeSkin() {
    uploading.value = true
    try {
        await removeMySkin()
        currentSkin.value = null
    } catch (err) {
        await handleSevereError(err)
    } finally {
        uploading.value = false
    }
}

async function removeCape() {
    uploadingCape.value = true
    try {
        await removeMyCape()
        currentCape.value = null
    } catch (err) {
        await handleSevereError(err)
    } finally {
        uploadingCape.value = false
    }
}

// Expose refresh for the router to call on navigation
defineExpose({ refresh })

onMounted(refresh)

const messages = defineMessages({
    title: { id: 'l3zawa-skins.title', defaultMessage: 'L3zawa Skins' },
    subtitle: {
        id: 'l3zawa-skins.subtitle',
        defaultMessage: 'Custom skins & capes visible to all L3zawa Launcher users',
    },
    serverOnline: { id: 'l3zawa-skins.server-online', defaultMessage: 'Server online' },
    serverOffline: { id: 'l3zawa-skins.server-offline', defaultMessage: 'Server offline' },
    noAccountTitle: { id: 'l3zawa-skins.no-account-title', defaultMessage: 'No active account' },
    noAccountDesc: {
        id: 'l3zawa-skins.no-account-desc',
        defaultMessage: 'Sign in or add an offline account to upload skins.',
    },
    noSkinYet: { id: 'l3zawa-skins.no-skin', defaultMessage: 'Upload a skin to get started' },
    skinSection: { id: 'l3zawa-skins.skin-section', defaultMessage: 'Skin' },
    capeSection: { id: 'l3zawa-skins.cape-section', defaultMessage: 'Cape' },
    model: { id: 'l3zawa-skins.model', defaultMessage: 'Model' },
    classic: { id: 'l3zawa-skins.classic', defaultMessage: 'Classic' },
    slim: { id: 'l3zawa-skins.slim', defaultMessage: 'Slim' },
    uploadSkin: { id: 'l3zawa-skins.upload-skin', defaultMessage: 'Upload skin' },
    uploadCape: { id: 'l3zawa-skins.upload-cape', defaultMessage: 'Upload cape' },
    remove: { id: 'l3zawa-skins.remove', defaultMessage: 'Remove' },
    currentSkin: { id: 'l3zawa-skins.current-skin', defaultMessage: 'Current skin' },
    currentCape: { id: 'l3zawa-skins.current-cape', defaultMessage: 'Current cape' },
    uploadedOn: { id: 'l3zawa-skins.uploaded-on', defaultMessage: 'Uploaded' },
    howItWorksTitle: { id: 'l3zawa-skins.how-title', defaultMessage: 'How it works' },
    howStep1: {
        id: 'l3zawa-skins.how-1',
        defaultMessage: 'Upload your skin and/or cape above.',
    },
    howStep2: {
        id: 'l3zawa-skins.how-2',
        defaultMessage: 'Launch any instance — the CustomSkinLoader mod is auto-injected.',
    },
    howStep3: {
        id: 'l3zawa-skins.how-3',
        defaultMessage: 'The mod fetches skins from the L3zawa skin server instead of Mojang.',
    },
    howStep4: {
        id: 'l3zawa-skins.how-4',
        defaultMessage: 'Anyone using L3zawa Launcher sees your skin in-game — and you see theirs.',
    },
})
</script>

<style scoped>
.l3zawa-skins-page {
    padding: 1.5rem;
    max-width: 900px;
    margin: 0 auto;
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
}

.header-section {
    display: flex;
    justify-content: space-between;
    align-items: center;
    flex-wrap: wrap;
    gap: 1rem;
}

.brand-logo {
    width: 48px;
    height: 48px;
    border-radius: 12px;
    background: var(--color-brand-highlight);
    display: flex;
    align-items: center;
    justify-content: center;
}

.no-account-card {
    display: flex;
    align-items: center;
    gap: 1rem;
    padding: 1.25rem;
    background: var(--color-bg);
    border: 1px solid var(--color-border);
    border-radius: 12px;
}

.preview-card {
    position: relative;
    height: 420px;
    background: var(--color-bg);
    border: 1px solid var(--color-border);
    border-radius: 16px;
    overflow: hidden;
    display: flex;
    align-items: center;
    justify-content: center;
}

.preview-renderer {
    width: 100%;
    height: 100%;
}

.preview-placeholder {
    display: flex;
    flex-direction: column;
    align-items: center;
}

.nametag-overlay {
    position: absolute;
    bottom: 1rem;
    left: 50%;
    transform: translateX(-50%);
    padding: 0.25rem 1rem;
    background: rgba(0, 0, 0, 0.5);
    border-radius: 999px;
}

.upload-section {
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
}

.section-title {
    font-size: 1.125rem;
    font-weight: 700;
    color: var(--color-text-default);
    margin: 0;
}

.upload-card {
    padding: 1.25rem;
    background: var(--color-bg);
    border: 1px solid var(--color-border);
    border-radius: 12px;
    display: flex;
    flex-direction: column;
    gap: 1rem;
}

.upload-controls {
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
}

.variant-label {
    font-size: 0.875rem;
    font-weight: 600;
    color: var(--color-text-default);
}

.variant-buttons {
    display: flex;
    gap: 0.5rem;
}

.action-buttons {
    display: flex;
    gap: 0.5rem;
    flex-wrap: wrap;
}

.hidden-file-input {
    display: none;
}

.current-info {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    padding-top: 0.75rem;
    border-top: 1px solid var(--color-border);
}

.skin-thumbnail {
    width: 48px;
    height: 48px;
    image-rendering: pixelated;
    border-radius: 6px;
    border: 1px solid var(--color-border);
}

.cape-thumbnail {
    width: 32px;
    height: 48px;
    image-rendering: pixelated;
    border-radius: 4px;
    border: 1px solid var(--color-border);
}

.how-it-works-card {
    padding: 1.25rem;
    background: var(--color-brand-highlight);
    border: 1px solid var(--color-brand);
    border-radius: 12px;
}
</style>
