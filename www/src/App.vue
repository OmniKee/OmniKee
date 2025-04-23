<template>
  <q-layout view="lHh Lpr lFf">
    <q-header data-tauri-drag-region>
      <q-toolbar class="titlebar" data-tauri-drag-region>
        <q-avatar data-tauri-drag-region="">
          <img src="@/assets/logo.svg" alt="OmniKee logo" />
        </q-avatar>
        <q-toolbar-title data-tauri-drag-region shrink>
          OmniKee
        </q-toolbar-title>

        <q-btn flat round dense>
          <q-icon name="mdi-content-save" />
          <q-tooltip>Save Database</q-tooltip>
        </q-btn>

        <q-btn flat round dense>
          <q-icon name="mdi-lock" />
          <q-tooltip>Lock Database</q-tooltip>
        </q-btn>

        <q-separator dark vertical spaced inset />

        <q-btn flat round dense>
          <q-icon name="mdi-plus" />
          <q-tooltip>Add Entry</q-tooltip>
        </q-btn>

        <q-btn flat round dense>
          <q-icon name="mdi-pencil" />
          <q-tooltip>Edit Entry</q-tooltip>
        </q-btn>

        <q-btn flat round dense>
          <q-icon name="mdi-delete" />
          <q-tooltip>Delete Entry</q-tooltip>
        </q-btn>

        <q-space />

        <q-input v-model="searchText" clearable dense filled style="max-width: 200px">
          <template #prepend>
            <q-icon name="mdi-magnify" />
          </template>
        </q-input>

        <window-buttons />

      </q-toolbar>

      <q-tabs align="left" class="bg-secondary">
        <q-route-tab :to="{name: '/'}">
          <q-icon name="mdi-folder-open" alt="Open Database" />
          <q-tooltip>Open&nbsp;Database</q-tooltip>
        </q-route-tab>

        <q-route-tab v-for="db, i in databases" :key="i" :to="{name: '/database/[i]/', params: {i}}" :label="db.name"
          no-caps />
      </q-tabs>
    </q-header>

    <q-page-container>
      <router-view />
    </q-page-container>
  </q-layout>
</template>

<script setup lang="ts">
import {computed, ref} from 'vue';

import {useDatabasesStore} from '@/stores/databases'

const databasesStore = useDatabasesStore()

const databases = computed(() => {
  return databasesStore.databases
})

const searchText = ref("")

</script>

<style scoped lang="scss">
.titlebar {
  user-select: none;
}

.tab-title {
  text-transform: none;
}
</style>
