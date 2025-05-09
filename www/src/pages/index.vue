<template>
  <q-page class="row items-center justify-evenly">

    <img class="background" src="@/assets/logo.svg" />

    <q-form style="width: 80%" @submit.prevent.stop="onSubmit">
      <q-card>
        <q-card-section>
          <div class="text-h6">Open Database</div>

          <q-file ref="$database" v-model="database" name="database" accept=".kdbx" label="Database" clearable filled
            class="q-mt-sm">
            <template #prepend>
              <q-icon name="mdi-file-account" />
            </template>
          </q-file>
          <q-input ref="$password" v-model="password" name="password" :type="showPassword ? 'text' : 'password'"
            label="Password" clearable filled class="password q-mt-sm">
            <template #prepend>
              <q-icon name="mdi-key-variant" />
            </template>

            <template #append>
              <q-btn flat :icon="showPassword ? 'mdi-eye' : 'mdi-eye-off'" @click="showPassword = !showPassword" />
            </template>
          </q-input>
          <q-file v-model="keyFile" name="keyFile" label="Key file" clearable filled class="q-mt-sm">
            <template #prepend>
              <q-icon name="mdi-file-key" />
            </template>

          </q-file>

        </q-card-section>

        <q-card-actions>
          <div class="bg-negative q-pa-sm q-ml-sm rounded-borders" v-if="errorMessage">{{ errorMessage }}</div>
          <q-space />
          <q-btn-group>
            <q-btn padding="sm lg" :disabled="loading" @click="onLoadExample">Load Example</q-btn>
            <q-btn padding="sm lg" :loading="loading" color="primary" type="submit">Open</q-btn>
            <q-btn padding="sm lg" :disabled="loading">Reset</q-btn>
          </q-btn-group>
        </q-card-actions>
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

import demoDB from '@/assets/demo.kdbx?uint8array'

import {ref, useTemplateRef, watch} from 'vue'
import {useRouter} from 'vue-router'

import {useDatabasesStore} from '@/stores/databases'
import {useAppStore} from '@/stores/app'
import {QFile, QInput} from 'quasar'


const app = useAppStore()
const databasesStore = useDatabasesStore()
const router = useRouter()

const database = ref(null)
const password = ref("")
const keyFile = ref(null)

const loading = ref(false)
const showPassword = ref(false)

const errorMessage = ref("")

const $database = useTemplateRef<QFile>("$database")
const $password = useTemplateRef<QInput>("$password")

watch(database, () => {
  if (database.value && $password.value) {
    $password.value.focus()
  } else if (!database.value && $database.value) {
    $database.value.focus()
  }
})

setTimeout(() => {if ($database.value) {$database.value.focus()} }, 100)

function readFile(file: File): Promise<Uint8Array> {
  return new Promise((resolve, reject) => {
    const reader = new FileReader()

    reader.onload = function (e) {
      if (!e.target) {return }
      const buffer = new Uint8Array(e.target.result as ArrayBuffer)
      resolve(buffer)
    }

    reader.onerror = function () {
      reject(new Error(`Failed reading: ${file.name}`))
    }

    reader.readAsArrayBuffer(file)
  })
}

async function onSubmit() {
  if (!database.value) {return }

  errorMessage.value = ""
  loading.value = true

  try {

    const databaseBuffer = await readFile(database.value)
    const keyFileBuffer = keyFile.value ? await readFile(keyFile.value) : null

    await databasesStore.loadDatabase(databaseBuffer, password.value, keyFileBuffer)

    await router.push({name: '/database/[i]/', params: {i: databasesStore.databases.length - 1}})
  } catch (e) {
    if (typeof e === 'string') {
      errorMessage.value = e
    } else if (e instanceof Error) {
      errorMessage.value = e.message
    }
  }

  loading.value = false

}

async function onLoadExample() {
  loading.value = true
  await databasesStore.loadDatabase(demoDB, "demopass", null)
  await router.push({name: '/database/[i]/', params: {i: databasesStore.databases.length - 1}})
  loading.value = false
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

.password {
  font-family: monospace;
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
