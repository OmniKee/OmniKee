<template>
  <q-page class="row">
    <q-splitter v-model="splitter" class="col-grow">
      <template #before>
        <q-tree v-model:selected="selectedNode" :nodes="nodes" node-key="uuid" selected-color="secondary"
          default-expand-all></q-tree>

      </template>
      <template #after>
        {{ selectedNode }}
      </template>

    </q-splitter>
  </q-page>
</template>

<script setup lang="ts">
import {computed, ref} from 'vue'
import {useRoute} from 'vue-router'

import {useDatabasesStore} from '@/stores/databases'
import {type Folder} from '@/omnikee'


const route = useRoute('/database/[i]')

const databasesStore = useDatabasesStore()

const database = computed(() => databasesStore.databases[+route.params.i])

const splitter = ref(30)

const selectedNode = ref<string | null>(null)


type TreeNode = {
  label: string,
  uuid: string,
  children: TreeNode[],
}

const nodes = computed(() => {
  if (!database.value) {return []}

  function translate(node: Folder): TreeNode {
    return {
      label: node.name,
      uuid: node.uuid,
      children: node.children.map(translate)
    }
  }
  return [translate(database.value.root)]
})

</script>
