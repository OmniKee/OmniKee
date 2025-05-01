<template>
  <q-input v-if="totp" :modelValue="totp.code" :type="reveal ? 'text' : 'password'" label="TOTP" readonly>
    <template #prepend>
      <q-circular-progress show-value :value="totp.valid_for.secs" :max="totp.period.secs" size="24px"
        font-size="12px" />
    </template>
    <template #append>
      <slot name="append" :totp="totp"></slot>
      <q-btn flat :icon="reveal ? 'mdi-eye' : 'mdi-eye-off'" @click="reveal = !reveal">
        <q-tooltip>Show/hide protected value</q-tooltip>
      </q-btn>

      <q-btn flat icon="mdi-content-copy" @click="onCopy">
        <q-tooltip>Copy to Clipboard</q-tooltip>
      </q-btn>
    </template>
  </q-input>
</template>

<script setup lang="ts">
import {ref} from 'vue'
import {asyncComputed, useNow} from '@vueuse/core'

import {type Entry, type OTPResponse} from 'omnikee-wasm'
import ok from '@/omnikee'

import {useViewStore} from '@/stores/view';

const props = defineProps<{
  entry: Entry,
}>()

const now = useNow()

const viewStore = useViewStore()


const reveal = ref(false)


const totp = asyncComputed<OTPResponse | undefined>(async () => {
  if (typeof viewStore.current.database === 'undefined') {return undefined}

  const time = BigInt(Math.floor(+now.value / 1000))
  return await ok.getOtp(viewStore.current.database, props.entry.uuid, time)
}, undefined)

async function onCopy() {
  if (typeof viewStore.current.database === "undefined" || !props.entry || !totp.value) {return }
  await navigator.clipboard.writeText(totp.value.code)
}

</script>

<style lang="scss" scoped>
.protected {
  font-family: monospace;
}
</style>
