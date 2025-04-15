<template>
  <q-layout view="lHh Lpr lFf">
    <q-header elevated data-tauri-drag-region>
      <q-toolbar class="titlebar" data-tauri-drag-region>
        <q-avatar data-tauri-drag-region="">
          <img src="@/assets/logo.svg" alt="OmniKee logo" />
        </q-avatar>
        <q-toolbar-title data-tauri-drag-region shrink>
          OmniKee
        </q-toolbar-title>

        <q-btn flat round dense>
          <q-icon name="mdi-folder-open" />
          <q-tooltip>Open Database</q-tooltip>
        </q-btn>


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

        <window-buttons />

      </q-toolbar>

      <q-tabs align="left" dense class="bg-secondary">
        <q-tab v-for="db, i in databases" :key="i" :name="db">
          <div class="tab-title">{{ db }}&nbsp;<q-btn dense flat round icon="mdi-close" size="sm" /></div>
        </q-tab>
      </q-tabs>
    </q-header>

    <q-page-container>
      <router-view />
    </q-page-container>
  </q-layout>
</template>

<script setup lang="ts">

const databases = ["asd", "fgh", "jkl"]

</script>

<style scoped lang="scss">
.titlebar {
  user-select: none;
}

.tab-title {
  text-transform: none;
}
</style>
