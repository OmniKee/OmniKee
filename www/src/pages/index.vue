<template>

  <q-page class="row items-center justify-evenly">
    <img class="background" src="@/assets/logo.svg" />

    <q-form style="width: 80%">
      <q-card class="no-padding">
        <q-card-section class="no-padding row">

          <q-btn color="primary" class="q-pa-sm col-grow" size="lg" stack icon="mdi-folder-open" @click="onLoad">Load
            Database</q-btn>
          <q-btn class="q-pa-sm col-grow" size="lg" stack icon="mdi-folder-open-outline" @click="onLoadDemo">Load
            Example</q-btn>

        </q-card-section>
      </q-card>
    </q-form>


    <q-footer class="q-pa-md row justify-end">
      <div>
        platform: {{ app.platform }}
      </div>
      <q-separator class="q-mx-sm" vertical />
      <div>
        <a href="https://omnikee.github.io/docs/getting-started/quick-start/" target="_blank">Docs</a>
      </div>
    </q-footer>
  </q-page>
</template>

<script setup lang="ts">

import {useRouter} from 'vue-router'

import {useDatabasesStore} from '@/stores/databases'
import {useViewStore} from '@/stores/view'
import {useAppStore} from '@/stores/app'

const router = useRouter()

const app = useAppStore()
const databasesStore = useDatabasesStore()
const viewStore = useViewStore()

viewStore.current.database = undefined


async function onLoad() {
  await databasesStore.loadDatabase()
  await router.push({name: '/database/[i]/', params: {i: databasesStore.databases.length - 1}})
}

async function onLoadDemo() {
  await databasesStore.loadDemo()
  await router.push({name: '/database/[i]/', params: {i: databasesStore.databases.length - 1}})
}

</script>

<style lang="scss" scoped>
.background {
  position: fixed;
  top: 0;
  left: 0;
  width: 80%;
  opacity: 0.1;
}

.q-footer a {
  color: inherit;
  text-decoration: none;
  font-weight: bold;

  &:hover {
    text-decoration: underline;
  }
}
</style>
