<template>
  <q-list>

    <q-item clickable @click="onOpen">
      <q-item-section avatar><q-avatar icon="mdi-folder-open" /></q-item-section>
      <q-item-section>Open...</q-item-section>
    </q-item>

    <q-separator />

    <q-item clickable @click="onSave" :disable="!isOpenDatabaseSelected" v-if="appStore.is.tauri">
      <q-item-section avatar><q-avatar icon="mdi-content-save" /></q-item-section>
      <q-item-section>Save (EXPERIMENTAL)</q-item-section>
    </q-item>

    <q-item clickable @click="onSaveAs" :disable="!isOpenDatabaseSelected">
      <q-item-section avatar><q-avatar icon="mdi-content-save-settings" /></q-item-section>
      <q-item-section>Save as... (EXPERIMENTAL)</q-item-section>
    </q-item>

    <q-separator />

    <q-item clickable @click="onLock" :disable="!isOpenDatabaseSelected">
      <q-item-section avatar><q-avatar icon="mdi-lock" /></q-item-section>
      <q-item-section>Lock</q-item-section>
    </q-item>

    <q-item clickable @click="onClose" :disable="typeof viewStore.current.database === 'undefined'">
      <q-item-section avatar></q-item-section>
      <q-item-section>Close</q-item-section>
    </q-item>

  </q-list>
</template>

<script setup lang="ts">
import {computed} from 'vue'
import {useRouter} from 'vue-router'

import {useAppStore} from '@/stores/app'
import {useDatabasesStore} from '@/stores/databases'
import {useViewStore} from '@/stores/view'

const router = useRouter()

const appStore = useAppStore()
const databasesStore = useDatabasesStore()
const viewStore = useViewStore()

const isOpenDatabaseSelected = computed(() => {
  return typeof viewStore.current.database !== 'undefined' && !!viewStore.database
})

async function onOpen() {
  await databasesStore.loadDatabase()
  await router.push({name: '/database/[i]/', params: {i: databasesStore.databases.length - 1}})
}

async function onSave() {
  if (typeof viewStore.current.database === 'undefined' || !viewStore.database) {return }
  await databasesStore.saveDatabase(viewStore.current.database)
}

async function onSaveAs() {
  if (typeof viewStore.current.database === 'undefined' || !viewStore.database) {return }
  await databasesStore.saveDatabaseAs(viewStore.current.database)
}

async function onLock() {
  if (typeof viewStore.current.database === 'undefined') {return }
  await databasesStore.lockDatabase(viewStore.current.database)
}

async function onClose() {

}
</script>
