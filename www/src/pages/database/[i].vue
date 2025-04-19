<template>
  <q-page class="row">
    <q-splitter v-model="splitter" class="col-grow">
      <template #before>
        <h1>{{ database?.name }}</h1>
      </template>
      <template #after>
        <h1>Data Data</h1>
      </template>

    </q-splitter>
  </q-page>
</template>

<script setup lang="ts">
import {computed, ref} from 'vue'
import {useRoute} from 'vue-router'

import {useDatabasesStore} from 'src/stores/databases'

const route = useRoute('/database/[i]')

const databasesStore = useDatabasesStore()

const database = computed(() => databasesStore.databases[+route.params.i])

const splitter = ref(30)

</script>
