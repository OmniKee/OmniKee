<template>
  <q-input :class="{protected: field?.type === 'Protected'}" :modelValue="field?.value" :type="type" readonly>
    <template #prepend>
      <slot name="prepend" :field="field"></slot>
    </template>
    <template #append>

      <slot name="append" :field="field"></slot>
      <q-btn v-if="field?.type === 'Protected'" flat :icon="reveal ? 'mdi-eye' : 'mdi-eye-off'"
        @click="reveal = !reveal">
        <q-tooltip>Show/hide protected value</q-tooltip>
      </q-btn>

      <q-btn v-if="props.copy" flat icon="mdi-content-copy" @click="onCopy">
        <q-tooltip>Copy to Clipboard</q-tooltip>
      </q-btn>
    </template>
  </q-input>
</template>

<script setup lang="ts">
import {computed, ref} from 'vue'
import {asyncComputed} from '@vueuse/core'

import {type Entry} from 'omnikee-wasm'
import ok from '@/omnikee'

import {useViewStore} from '@/stores/view';

const props = defineProps<{
  entry: Entry,
  field: string,
  copy?: boolean,
}>()

const viewStore = useViewStore()

const reveal = ref(false)

type Field = {
  type: undefined | 'Protected' | 'Unprotected' | 'Bytes',
  value?: string,
}

const field = asyncComputed<Field | undefined>(async () => {
  if (typeof viewStore.current.database === 'undefined') {return undefined}

  const fieldValue = 'get' in props.entry.fields ? props.entry.fields.get(props.field) : props.entry.fields[props.field]
  const revealValue = reveal.value

  if (typeof fieldValue === 'undefined') {
    return {type: undefined}

  } else if (fieldValue === 'Protected') {
    const value = await ok.revealProtected(viewStore.current.database, props.entry.uuid, props.field)
    if (revealValue && typeof value !== 'undefined') {
      return {type: 'Protected', value}
    } else if (typeof value !== 'undefined') {
      return {type: 'Protected', value: '*'.repeat(value.length)}
    } else {
      return {type: 'Protected'}
    }
  } else if ('Unprotected' in fieldValue) {
    return {type: 'Unprotected', value: fieldValue['Unprotected']}
  } else if ('Bytes' in fieldValue) {
    return {type: 'Bytes', value: '(binary data)'}
  }
}, undefined)

const type = computed(() => {
  return field.value?.type === 'Protected' && !reveal.value ? 'password' : 'text'
})

async function onCopy() {
  if (typeof viewStore.current.database === "undefined" || !props.entry || !field.value) {return }

  if (field.value.type === 'Protected') {
    const pwd = await ok.revealProtected(viewStore.current.database, props.entry.uuid, props.field)
    if (typeof pwd === "undefined") {return }
    await navigator.clipboard.writeText(pwd)

  } else if (field.value.type === 'Unprotected') {
    if (typeof field.value.value === "undefined") {return }
    await navigator.clipboard.writeText(field.value.value)
  }
}

</script>

<style lang="scss" scoped>
.protected {
  font-family: monospace;
}
</style>
