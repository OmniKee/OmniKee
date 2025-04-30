<template>
  <q-toolbar class="bg-accent">
    <q-btn flat icon="mdi-arrow-left-circle" :to="{name: '/database/[i]/', params: {i: route.params.i}}" />
    <q-toolbar-title class="text-subtitle2" shrink>{{ entry?.name || route.params.uuid }}</q-toolbar-title>
  </q-toolbar>

  <q-page class="row" v-if="entry">

    <q-splitter v-model="splitter" unit="px" :limits="[100, Infinity]" class="col-grow">

      <template #before>
        <q-tabs v-model="tab" vertical>
          <q-tab name="entry" icon="mdi-pencil" label="Entry" />
          <q-tab name="advanced" icon="mdi-text-box-edit" label="Advanced" />
        </q-tabs>
      </template>


      <template #after>
        <q-tab-panels v-model="tab" animated swipeable vertical transition-prev="jump-up" transition-next="jump-up">

          <q-tab-panel name="entry">

            <FieldInput class="q-mb-sm" filled :entry="entry" field="Title" label="Title" />
            <FieldInput copy class="q-mb-sm" filled :entry="entry" field="UserName" label="Username" />
            <FieldInput copy class="q-mb-sm" filled :entry="entry" field="Password" label="Password" />
            <FieldInput copy class="q-mb-sm" filled :entry="entry" field="URL" label="URL">
              <template #append="{field}">
                <q-btn flat v-if="field?.value" @click="onOpen(field.value)" icon="mdi-open-in-new" target="_blank">
                  <q-tooltip>Open in Browser</q-tooltip>
                </q-btn>
              </template>

            </FieldInput>

          </q-tab-panel>

          <q-tab-panel name="advanced" v-if="entry">

            <h1> Advanced</h1>

          </q-tab-panel>

        </q-tab-panels>
      </template>

    </q-splitter>
  </q-page>
</template>

<script setup lang="ts">
import {computed, ref} from 'vue'
import {useRoute} from 'vue-router'

import {useViewStore} from '@/stores/view'

import ok from '@/omnikee'

const route = useRoute('/database/[i]/entry/[uuid]')

const viewStore = useViewStore()

viewStore.current.database = +route.params.i
viewStore.current.entry = route.params.uuid

const splitter = ref(100)
const entry = computed(() => viewStore.entry)

const tab = ref("entry")

async function onOpen(url: string) {
  await ok.openExternalLink(url)
}

</script>
