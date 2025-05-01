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
        <q-tab-panels style="height: 100%" v-model="tab" animated swipeable vertical transition-prev="jump-up"
          transition-next="jump-up">

          <q-tab-panel name="entry">

            <FieldInput class="q-mb-sm" filled :entry="entry" field="Title" label="Title">
              <template #prepend><q-icon name="mdi-form-textbox" /></template>
            </FieldInput>

            <FieldInput copy class="q-mb-sm" filled :entry="entry" field="UserName" label="Username">
              <template #prepend><q-icon name="mdi-account" /></template>
            </FieldInput>

            <FieldInput copy class="q-mb-sm" filled :entry="entry" field="Password" label="Password">
              <template #prepend><q-icon name="mdi-key" /></template>
            </FieldInput>

            <TOTPField class="q-mb-sm" filled :entry="entry" />

            <FieldInput copy class="q-mb-sm" filled :entry="entry" field="URL" label="URL">
              <template #prepend><q-icon name="mdi-web" /></template>
              <template #append="{field}">
                <q-btn flat v-if="field?.value" @click="onOpen(field.value)" icon="mdi-open-in-new" target="_blank">
                  <q-tooltip>Open in Browser</q-tooltip>
                </q-btn>
              </template>

            </FieldInput>

          </q-tab-panel>

          <q-tab-panel class="row no-padding" name="advanced" v-if="entry">
            <q-splitter class="col-grow" v-model="advancedSplit" unit="px" :limits="[100, Infinity]">
              <template #before>
                <q-list>
                  <q-item active-class="bg-primary text-white" v-for="field, i in fields" :key="i" clickable
                    :active="selectedField == field.field" @click="selectedField = field.field">
                    <q-item-section avatar>
                      <q-avatar :icon="field.icon" />
                    </q-item-section>

                    <q-item-section>
                      {{ field.field }}
                    </q-item-section>
                  </q-item>

                </q-list>
              </template>

              <template #after>
                <div class="q-pa-sm">
                  <FieldInput copy v-if="selectedField" :entry="entry" :field="selectedField" :label="selectedField" />
                </div>
              </template>

            </q-splitter>
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
import {type Value} from 'omnikee-wasm'

const route = useRoute('/database/[i]/entry/[uuid]')

const viewStore = useViewStore()

viewStore.current.database = +route.params.i
viewStore.current.entry = route.params.uuid

const splitter = ref(100)
const entry = computed(() => viewStore.entry)

const tab = ref("entry")

const advancedSplit = ref(200)

async function onOpen(url: string) {
  await ok.openExternalLink(url)
}

const defaultFields = new Set(['UserName', 'Password', 'Title', 'URL'])

const selectedField = ref<string | undefined>(undefined)

const fields = computed(() => {
  if (!entry.value) {return []}

  const entries: [string, Value][] = ('entries' in entry.value.fields ? [...entry.value.fields.entries()] : Object.entries(entry.value.fields))
  return entries.map(([field, fieldValue]) => {
    if (fieldValue === 'Protected') {
      return {field, type: 'Protected', icon: 'mdi-lock'}

    } else if ('Unprotected' in fieldValue) {
      return {field, type: 'Unprotected', icon: 'mdi-text'}

    } else if ('Bytes' in fieldValue) {
      return {field, type: 'Bytes', icon: 'mdi-file'}
    }
  }).filter((e) => typeof e !== 'undefined')
    .filter((e) => !defaultFields.has(e.field))
})

</script>
