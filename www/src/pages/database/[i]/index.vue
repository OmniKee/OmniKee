<template>
  <q-page class="row">
    <q-splitter v-model="splitter" class="col-grow">
      <template #before>
        <q-scroll-area style="height: 100%; max-width: 100%;">
          <q-tree v-model:selected="selectedGroup" :nodes="nodes" node-key="uuid" selected-color="accent"
            default-expand-all dense>
            <template #default-header="{node}">
              <q-avatar size="lg" v-if="node.avatar" :icon="node.avatar" />

              {{ node.label }}

            </template>

          </q-tree>
        </q-scroll-area>

      </template>
      <template #after>
        <q-scroll-area style="height: 100%; max-width: 100%;">
          <q-table class="entries" v-if="viewStore.groupEntries" :rows="viewStore.groupEntries"
            :loading="viewStore.loadingGroupEntries" :columns="columns" row-key="uuid" :rows-per-page-options="[0]"
            hide-pagination @row-click="onRowClick">

            <template #body-cell-name="{row}">
              <q-td @dblclick="onDoubleClick(row)">
                <q-avatar size="lg" v-if="row.icon"
                  :icon="row.icon.startsWith('mdi-') ? row.icon : `img:${row.icon}`" />
                {{ row.name }}
              </q-td>
            </template>

          </q-table>
        </q-scroll-area>
      </template>

    </q-splitter>
  </q-page>
</template>

<script setup lang="ts">
import {computed, ref} from 'vue'
import {useRoute, useRouter} from 'vue-router'
import {type QTableColumn, type QTreeNode} from 'quasar'

import {useViewStore} from '@/stores/view'

import {type Entry, type Group} from 'omnikee-wasm'

const route = useRoute('/database/[i]/')
const router = useRouter()

const viewStore = useViewStore()

viewStore.current.database = +route.params.i

const splitter = ref(20)

const nodes = computed(() => {
  if (!viewStore.database) {return []}

  function translate(node: Group): QTreeNode {

    const out: QTreeNode = {
      label: node.name,
      uuid: node.uuid,
      children: node.children.map(translate)
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
  {name: "name", label: "Name", field: "name", align: "left", sortable: true},
  {name: "user_name", label: "Username", field: "user_name", align: "left", sortable: true},
  {name: "url", label: "URL", field: "url", align: "left", sortable: true}
]

function onRowClick(_event: Event, entry: Entry) {
  viewStore.current.entry = entry.uuid
}

async function onDoubleClick(entry: Entry) {
  viewStore.current.entry = entry.uuid
  await router.push({name: '/database/[i]/entry/[uuid]', params: {i: route.params.i, uuid: entry.uuid}})
}

</script>
