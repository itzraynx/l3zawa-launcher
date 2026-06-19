<template>
    <ModalWrapper ref="modal" :header="formatMessage(messages.title)" :show-ad-on-close="false">
        <div class="flex flex-col gap-4 p-1">
            <p class="m-0 text-sm text-secondary leading-tight">
                {{ formatMessage(messages.description) }}
            </p>

            <div class="flex flex-col gap-2">
                <label class="m-0 text-sm font-semibold text-primary" for="offline-username">
                    {{ formatMessage(messages.usernameLabel) }}
                </label>
                <div class="flex items-center gap-2">
                    <StyledInput
                        id="offline-username"
                        v-model="username"
                        type="text"
                        :placeholder="formatMessage(messages.usernamePlaceholder)"
                        wrapper-class="flex-1"
                        :maxlength="16"
                        :aria-invalid="!!error"
                        @keyup.enter="submit"
                    />
                    <ButtonStyled type="outlined" circular>
                        <button
                            v-tooltip="formatMessage(messages.randomUsername)"
                            :aria-label="formatMessage(messages.randomUsername)"
                            @click="generateRandomUsername"
                        >
                            <SparklesIcon />
                        </button>
                    </ButtonStyled>
                </div>
                <p
                    v-if="error"
                    class="m-0 text-sm text-red leading-tight"
                    role="alert"
                >
                    {{ error }}
                </p>
                <p v-else class="m-0 text-xs text-secondary leading-tight">
                    {{ formatMessage(messages.usernameHint) }}
                </p>
            </div>

            <div class="flex justify-end gap-2 pt-1">
                <ButtonStyled type="outlined">
                    <button class="!shadow-none" @click="cancel">
                        <XIcon />
                        {{ formatMessage(messages.cancel) }}
                    </button>
                </ButtonStyled>
                <ButtonStyled color="brand">
                    <button :disabled="!canSubmit || submitting" @click="submit">
                        <SpinnerIcon v-if="submitting" class="animate-spin" />
                        <LogInIcon v-else />
                        {{ formatMessage(messages.login) }}
                    </button>
                </ButtonStyled>
            </div>
        </div>
    </ModalWrapper>
</template>

<script setup lang="ts">
import { LogInIcon, SparklesIcon, SpinnerIcon, XIcon } from '@modrinth/assets'
import {
    ButtonStyled,
    defineMessages,
    StyledInput,
    useVIntl,
} from '@modrinth/ui'
import { computed, ref } from 'vue'

import ModalWrapper from '@/components/ui/modal/ModalWrapper.vue'
import { trackEvent } from '@/helpers/analytics'
import { login_offline } from '@/helpers/auth.js'
import { handleSevereError } from '@/store/error.js'

const { formatMessage } = useVIntl()

const emit = defineEmits<{
    success: [credentials: { profile: { id: string; name: string } }]
}>()

const modal = ref<InstanceType<typeof ModalWrapper> | null>(null)
const username = ref('')
const error = ref('')
const submitting = ref(false)

// Minecraft username rules: 3-16 chars, alphanumeric + underscore.
const USERNAME_REGEX = /^[A-Za-z0-9_]{3,16}$/

const canSubmit = computed(() => USERNAME_REGEX.test(username.value.trim()))

function validate(): boolean {
    const name = username.value.trim()
    if (name.length < 3) {
        error.value = formatMessage(messages.errorTooShort)
        return false
    }
    if (name.length > 16) {
        error.value = formatMessage(messages.errorTooLong)
        return false
    }
    if (!USERNAME_REGEX.test(name)) {
        error.value = formatMessage(messages.errorInvalidChars)
        return false
    }
    error.value = ''
    return true
}

async function submit() {
    if (!validate() || submitting.value) return

    submitting.value = true
    try {
        const credentials = await login_offline(
            username.value.trim(),
        ).catch(handleSevereError)

        if (credentials) {
            trackEvent('AccountLogInOffline')
            emit('success', credentials)
            modal.value?.hide()
            // Reset for next open
            username.value = ''
            error.value = ''
        }
    } finally {
        submitting.value = false
    }
}

function cancel() {
    modal.value?.hide()
    username.value = ''
    error.value = ''
}

// --- Bonus: random username generator ---
// Picks a recognizable adjective + noun combination so the generated name
// feels like a real gamer tag rather than random gibberish.
const ADJECTIVES = [
    'Bold', 'Calm', 'Clever', 'Crimson', 'Daring', 'Eager', 'Fierce', 'Frozen',
    'Golden', 'Happy', 'Iron', 'Jolly', 'Keen', 'Lucky', 'Mighty', 'Noble',
    'Oldest', 'Proud', 'Quick', 'Rapid', 'Silent', 'Tough', 'Unseen', 'Vivid',
    'Wily', 'Young', 'Zen', 'Brave', 'Chill', 'Dark',
]
const NOUNS = [
    'Wolf', 'Fox', 'Bear', 'Hawk', 'Lion', 'Tiger', 'Otter', 'Owl',
    'Raven', 'Stag', 'Viper', 'Whale', 'Yak', 'Cat', 'Dog', 'Falcon',
    'Goat', 'Hare', 'Ibis', 'Jay', 'Koala', 'Lynx', 'Moose', 'Newt',
    'Panda', 'Quail', 'Shark', 'Toad', 'Eagle', 'Cobra',
]

function generateRandomUsername() {
    const adj = ADJECTIVES[Math.floor(Math.random() * ADJECTIVES.length)]
    const noun = NOUNS[Math.floor(Math.random() * NOUNS.length)]
    const num = Math.floor(Math.random() * 100)
    // Keep total length <= 16 (Minecraft's max).
    let name = `${adj}${noun}`
    if (name.length + 2 <= 16) name += num.toString().padStart(2, '0')
    else if (name.length + 1 <= 16) name += num.toString()[0]
    username.value = name.slice(0, 16)
    error.value = ''
}

defineExpose({
    show: (e?: MouseEvent) => {
        username.value = ''
        error.value = ''
        submitting.value = false
        modal.value?.show(e)
    },
    hide: () => {
        modal.value?.hide()
    },
})

const messages = defineMessages({
    title: {
        id: 'offline-account.title',
        defaultMessage: 'Add new offline account',
    },
    description: {
        id: 'offline-account.description',
        defaultMessage:
            'Play Minecraft without a Microsoft account using a custom username. ' +
            'Works on offline-mode servers and singleplayer.',
    },
    usernameLabel: {
        id: 'offline-account.username-label',
        defaultMessage: 'Enter your player name',
    },
    usernamePlaceholder: {
        id: 'offline-account.username-placeholder',
        defaultMessage: 'Your player name here...',
    },
    usernameHint: {
        id: 'offline-account.username-hint',
        defaultMessage: '3-16 characters, letters, numbers, and underscores only.',
    },
    login: {
        id: 'offline-account.login',
        defaultMessage: 'Login',
    },
    cancel: {
        id: 'offline-account.cancel',
        defaultMessage: 'Cancel',
    },
    randomUsername: {
        id: 'offline-account.random-username',
        defaultMessage: 'Generate a random username',
    },
    errorTooShort: {
        id: 'offline-account.error.too-short',
        defaultMessage: 'Username must be at least 3 characters.',
    },
    errorTooLong: {
        id: 'offline-account.error.too-long',
        defaultMessage: 'Username must be at most 16 characters.',
    },
    errorInvalidChars: {
        id: 'offline-account.error.invalid-chars',
        defaultMessage: 'Only letters, numbers, and underscores are allowed.',
    },
    errorGeneric: {
        id: 'offline-account.error.generic',
        defaultMessage: 'Could not create offline account. Please try again.',
    },
})
</script>
