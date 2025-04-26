<template>
  <q-toolbar class="bg-accent">
    <q-btn flat icon="mdi-arrow-left-circle" :to="{name: '/database/[i]/', params: {i: route.params.i}}" />
    <q-toolbar-title class="text-subtitle2" shrink>{{ entry?.name || route.params.uuid }}</q-toolbar-title>
  </q-toolbar>

  <q-page class="row" v-if="entry">

    <q-splitter v-model="splitter" class="col-grow">

      <template #before>
        <q-tabs v-model="tab" vertical>
          <q-tab name="entry" icon="mdi-pencil" label="Entry" />
          <q-tab name="advanced" icon="mdi-text-box-edit" label="Advanced" />
        </q-tabs>
      </template>


      <template #after>
        <q-tab-panels v-model="tab" animated swipeable vertical transition-prev="jump-up" transition-next="jump-up">

          <q-tab-panel name="entry">

            <q-input class="q-mb-sm" v-model="entry.name" label="Title" filled />

            <q-input class="q-mb-sm" v-model="entry.user_name" label="Username" filled />

            <q-input class="q-mb-sm" v-model="password" :type="showPassword ? 'text' : 'password'" label="Password"
              filled readonly>
              <template #append>
                <q-btn flat :icon="showPassword ? 'mdi-eye' : 'mdi-eye-off'" @click="showPassword = !showPassword">
                  <q-tooltip>Show/Hide Password</q-tooltip>
                </q-btn>
                <q-btn flat icon="mdi-content-copy" @click="onCopy">
                  <q-tooltip>Copy to Clipboard</q-tooltip>
                </q-btn>


              </template>

            </q-input>

          </q-tab-panel>

          <q-tab-panel name="advanced">
            <h1>Advanced</h1>
          </q-tab-panel>

        </q-tab-panels>
      </template>

    </q-splitter>
  </q-page>
</template>

<script setup lang="ts">
import {computed, ref} from 'vue'
import {useRoute} from 'vue-router'
import {asyncComputed} from '@vueuse/core'

import {useViewStore} from '@/stores/view'

import ok from '@/omnikee'

const route = useRoute('/database/[i]/entry/[uuid]')

const viewStore = useViewStore()

viewStore.current.database = +route.params.i
viewStore.current.entry = route.params.uuid

const splitter = ref(10)
const entry = computed(() => viewStore.entry)


const tab = ref("entry")

const showPassword = ref(false)


const password = asyncComputed(async () => {
  if (typeof viewStore.current.database === "undefined" || !viewStore.current.entry) {return undefined}

  const show = showPassword.value
  const pwd = await ok.revealProtected(viewStore.current.database, viewStore.current.entry, "Password")

  if (show) {
    return pwd
  } else if (pwd) {
    return "*".repeat(pwd.length)
  } else {
    return undefined
  }

}, undefined)

async function onCopy() {
  if (typeof viewStore.current.database === "undefined" || !viewStore.current.entry) {return }
  const pwd = await ok.revealProtected(viewStore.current.database, viewStore.current.entry, "Password")

  if (typeof pwd === "undefined") {return }

  await navigator.clipboard.writeText(pwd)

}

</script>
