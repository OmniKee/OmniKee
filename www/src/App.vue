<template>
  <q-layout view="lHh Lpr lFf">
    <q-header data-tauri-drag-region>
      <q-toolbar class="titlebar q-pl-none" data-tauri-drag-region>

        <q-btn-dropdown class="row" no-caps auto-close>
          <template #label>
            <q-avatar>
              <img src="@/assets/logo.svg" alt="OmniKee logo" />
            </q-avatar>
            <q-toolbar-title data-tauri-drag-region shrink>
              OmniKee
            </q-toolbar-title>

          </template>


          <Menu />
        </q-btn-dropdown>

        <q-space />

        <q-input v-model="searchText" shadow-text="Search..." clearable dense disable filled style="max-width: 200px">
          <template #prepend>
            <q-icon name="mdi-magnify" />
          </template>
        </q-input>

        <window-buttons />

      </q-toolbar>

      <q-tabs align="left" class="bg-secondary">

        <q-btn-dropdown class="alternate" auto-close>
          <template #label>
            <img src="@/assets/logo.svg" alt="OmniKee logo" />
          </template>

          <Menu />
        </q-btn-dropdown>

        <q-route-tab class="q-px-sm" :to="{name: '/'}">
          <q-icon name="mdi-home" alt="Home" />
          <q-tooltip>Home</q-tooltip>
        </q-route-tab>

        <q-route-tab v-for="db, i in databases" :key="i" :to="{name: '/database/[i]/', params: {i}}" no-caps>

          <div>
            {{ db.name }}
            <q-btn class="col" flat round icon="mdi-close" size="xs"
              @click.prevent.stop="databasesStore.closeDatabase(i)" />
          </div>

        </q-route-tab>


        <q-space class="alternate" />
        <window-buttons class="alternate" />
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

.q-tabs .alternate {
  display: none;
}

body.screen--sm,
body.screen--xs {
  .q-header .q-toolbar {
    display: none;
  }

  .q-tabs {
    background-color: $primary !important;
  }

  .q-tabs .alternate {
    display: inline-block;
  }
}
</style>
