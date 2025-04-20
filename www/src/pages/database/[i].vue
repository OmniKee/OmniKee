<template>
  <q-page class="row">
    <q-splitter v-model="splitter" class="col-grow">
      <template #before>
        <q-scroll-area style="height: 100%; max-width: 100%;">
          <q-tree v-model:selected="selectedNode" :nodes="nodes" node-key="uuid" selected-color="secondary"
            default-expand-all></q-tree>
        </q-scroll-area>

      </template>
      <template #after>
        <!--<q-scroll-area style="height: 100%; max-width: 100%;">-->
        <q-table class="entries" v-if="entries" :rows="entries" :columns="columns" row-key="uuid"
          :rows-per-page-options="[0]" hide-pagination />
        <!--</q-scroll-area>-->
      </template>

    </q-splitter>
  </q-page>
</template>

<script setup lang="ts">
import {computed, ref} from 'vue'
import {useRoute} from 'vue-router'
import {asyncComputed} from '@vueuse/core'
import {type QTableColumn} from 'quasar'

import {useDatabasesStore} from '@/stores/databases'
import {type Entry, type Group} from 'omnikee-wasm'
import ok from '@/omnikee'


const route = useRoute('/database/[i]')

const databasesStore = useDatabasesStore()

const database = computed(() => databasesStore.databases[+route.params.i])

const splitter = ref(20)

const selectedNode = ref<string | null>(null)


type TreeNode = {
  label: string,
  uuid: string,
  children: TreeNode[],
}

const nodes = computed(() => {
  if (!database.value) {return []}

  function translate(node: Group): TreeNode {
    return {
      label: node.name,
      uuid: node.uuid,
      children: node.children.map(translate)
    }
  }
  return [translate(database.value.root)]
})


const columns: QTableColumn[] = [
  {name: "name", label: "Name", field: "name", align: "left", sortable: true},
  {name: "user_name", label: "Username", field: "user_name", align: "left", sortable: true},
  {name: "url", label: "URL", field: "url", align: "left", sortable: true}
]

const entries = asyncComputed(async () => {
  if (!selectedNode.value) {return []}
  const res: Entry[] = await ok.listEntries(+route.params.i, selectedNode.value)

  return res
}, undefined)


</script>
