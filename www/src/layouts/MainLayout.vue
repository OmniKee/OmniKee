<template>
  <q-layout view="lHh Lpr lFf">
    <q-header elevated data-tauri-drag-region>
      <q-toolbar class="titlebar" data-tauri-drag-region>
        <q-avatar data-tauri-drag-region="">
          <img src="@/assets/logo.svg" alt="OmniKee logo" />
        </q-avatar>
        <q-toolbar-title data-tauri-drag-region>
          OmniKee
        </q-toolbar-title>

        <window-buttons />

      </q-toolbar>
    </q-header>

    <q-page-container>
      <router-view />
    </q-page-container>
  </q-layout>
</template>

<script setup lang="ts">
</script>

<style scoped lang="scss">
.titlebar {
  user-select: none;
}
</style>
