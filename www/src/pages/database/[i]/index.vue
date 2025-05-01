<template>
  <q-page class="row">
    <q-splitter v-model="splitter" class="col-grow main-splitter">

      <template #before>
        <q-card style="height: 100%; max-width: 100%;">
          <q-scroll-area style="height: 100%; max-width: 100%;">
            <q-tree v-model:selected="selectedGroup" :nodes="nodes" node-key="uuid" selected-color="accent"
              default-expand-all dense>
              <template #default-header="{node}">
                <q-avatar size="lg" v-if="node.avatar" :icon="node.avatar" />
                {{ node.label }}
              </template>
            </q-tree>
          </q-scroll-area>
        </q-card>
      </template>

      <template #after>
        <q-scroll-area style="height: 100%; max-width: 100%;">
          <q-table class="entries" v-if="viewStore.groupEntries" :rows="viewStore.groupEntries"
            :loading="viewStore.loadingGroupEntries" :columns="columns" row-key="uuid" :rows-per-page-options="[0]"
            hide-pagination @row-click="onRowClick">

            <template #body-cell-icons="{row}">
              <q-td @dblclick="onDoubleClick(row)">
                <q-avatar size="md" :icon="hasField(row, 'otp') ? 'mdi-clock-outline' : ''">
                  <q-tooltip v-if="hasField(row, 'otp')">Has TOTP</q-tooltip>
                </q-avatar>
                <q-avatar size="lg" v-if="row.icon"
                  :icon="row.icon.startsWith('mdi-') ? row.icon : `img:${row.icon}`" />
              </q-td>
            </template>

            <template #body-cell-name="{row}">
              <q-td class="text-weight-bold" @dblclick="onDoubleClick(row)">
                {{ row.name }}
              </q-td>
            </template>

            <template #body-cell-username="{row}">
              <q-td @dblclick="onUsernameDoubleClick(row)">
                {{ row.user_name }}
              </q-td>
            </template>

            <template #body-cell-password="{row}">
              <q-td class="" @dblclick="onPasswordDoubleClick(row)">
                {{ hasField(row, 'Password') ? "&#x25cf;&#x25cf;&#x25cf;" : "" }}
              </q-td>
            </template>

            <template #body-cell-url="{row}">
              <q-td>
                <a :href="row.url" target="_blank">{{ row.url }}</a>
              </q-td>
            </template>

          </q-table>
        </q-scroll-area>
      </template>

    </q-splitter>

    <q-list class="alternate-layout col" v-if="listItems">

      <q-item clickable v-for="g in listItems.groups" :key="g.uuid" @click="onListGroupClick(g)">
        <q-item-section avatar>
          <q-avatar :icon="g.icon.startsWith('mdi-') ? g.icon : `img:${g.icon}`" v-if="g.icon" />
        </q-item-section>
        <q-item-section class="text-weight-bold">{{ g.name }}</q-item-section>
      </q-item>

      <q-separator />
      <q-item clickable v-for="e in listItems.entries" :key="e.uuid" @click="onDoubleClick(e)">
        <q-item-section avatar>
          <q-avatar :icon="e.icon.startsWith('mdi-') ? e.icon : `img:${e.icon}`" v-if="e.icon" />
        </q-item-section>
        <q-item-section>
          <q-item-label class="text-weight-bold">{{ e.name }}</q-item-label>
          <q-item-label caption v-if="e.user_name">{{ e.user_name }}</q-item-label>
        </q-item-section>
      </q-item>
    </q-list>


  </q-page>
</template>

<script setup lang="ts">
import {computed, ref, watch} from 'vue'

import {useRoute, useRouter} from 'vue-router'

import {type QTableColumn, type QTreeNode} from 'quasar'

import {useViewStore} from '@/stores/view'

import ok from '@/omnikee'
import {type Entry, type Group} from 'omnikee-wasm'
import {asyncComputed} from '@vueuse/core'

definePage({
  meta: {
    title() {
      const viewStore = useViewStore()
      return viewStore.database?.name || viewStore.current.database
    },
  },
})

const route = useRoute('/database/[i]/')
const router = useRouter()

const viewStore = useViewStore()


function updateRoute() {
  viewStore.current.database = +route.params.i
}

updateRoute()
watch(route, updateRoute)


const splitter = ref(20)

/** Nodes for the tree view in the normal layout */
const nodes = computed(() => {
  if (!viewStore.database) {return []}

  function translate(node: Group): QTreeNode {

    const out: QTreeNode = {
      label: node.name,
      uuid: node.uuid,
      children: (node.children || []).map(translate)
    }

    if (node.icon && node.icon.startsWith("mdi-")) {
      out.avatar = node.icon
    } else if (node.icon) {
      out.avatar = `img:${node.icon}`
    }

    return out
  }
  return [translate(viewStore.database.root)]
})

const selectedGroup = computed({
  get() {
    return viewStore.current.group || viewStore.database?.root.uuid
  },
  set(v) {
    if (typeof v === "undefined") {
      v = viewStore.database?.root.uuid
    }
    viewStore.current.group = v
  }
})


const columns: QTableColumn[] = [
  {name: "icons", label: "", field: "name", align: "left", sortable: false, headerStyle: "width: 50px"},
  {name: "name", label: "Name", field: "name", align: "left", sortable: true},
  {name: "user_name", label: "Username", field: "user_name", align: "left", sortable: true},
  {name: "password", label: "Password", field: "password", align: "left", sortable: false},
  {name: "url", label: "URL", field: "url", align: "left", sortable: true}
]

function onRowClick(_event: Event, entry: Entry) {
  viewStore.current.entry = entry.uuid
}

async function onDoubleClick(entry: Entry) {
  viewStore.current.entry = entry.uuid
  await router.push({name: '/database/[i]/entry/[uuid]', params: {i: route.params.i, uuid: entry.uuid}})
}

async function onUsernameDoubleClick(entry: Entry) {
  if (!entry.user_name) {return }
  await navigator.clipboard.writeText(entry.user_name)
}

async function onPasswordDoubleClick(entry: Entry) {
  if (typeof viewStore.current.database === 'undefined') {return }
  const pwd = await ok.revealProtected(viewStore.current.database, entry.uuid, "Password")

  if (typeof pwd === 'undefined') {return }
  await navigator.clipboard.writeText(pwd)
}

const listItems = asyncComputed(() => {
  if (!viewStore.database) {return {groups: [], entries: []}}

  function find(current: Group, parent: Group | undefined): {current: Group, parent: Group | undefined} | undefined {
    if (current.uuid === selectedGroup.value) {
      return {current, parent}
    }

    for (const child of current.children) {
      const res = find(child, current)
      if (typeof res !== "undefined") {return res}
    }
  }

  const found = find(viewStore.database.root, undefined)
  if (!found) {return {groups: [], entries: []}}

  const {current, parent} = found

  const groups: Group[] = current.children

  if (parent) {
    groups.unshift({
      uuid: parent.uuid,
      name: `[ up to ${parent.name} ]`,
      icon: parent.icon,
    } as Group)
  }

  const entries: Entry[] = (viewStore.groupEntries || [])

  return {groups, entries}
})

function onListGroupClick(group: Group) {
  viewStore.current.group = group.uuid
}

function hasField(entry: Entry, field: string) {
  // this is needed to support both WebAssembly (compiles fields into a Map) and Tauri (compiles it into an object)
  return (field in entry.fields) || ('has' in entry.fields && entry.fields.has(field))
}

</script>

<style lang="scss" scoped>
.alternate-layout {
  display: none;
}

body.screen--sm,
body.screen--xs {
  .main-splitter {
    display: none;
  }

  .alternate-layout {
    display: block;
  }
}
</style>
