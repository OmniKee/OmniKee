<template>
  <q-layout view="lHh Lpr lFf">
    <q-header elevated>
      <q-toolbar data-tauri-drag-region>
        <q-avatar>
          <img src="@/assets/logo.svg" alt="OmniKee logo" />
        </q-avatar>
        <q-toolbar-title>
          OmniKee
        </q-toolbar-title>

      </q-toolbar>
    </q-header>

    <q-page-container>
      <router-view />
    </q-page-container>
  </q-layout>
</template>

<script setup lang="ts">
</script>
