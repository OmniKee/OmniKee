<template>
  <q-page class="row items-center justify-evenly">

    <img class="background" src="@/assets/logo.svg" />

    <q-form style="width: 80%" @submit.prevent.stop="onSubmit">
      <q-card>
        <q-card-section>
          <div class="text-h6">Unlock Database</div>

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
            <q-btn padding="sm lg" :loading="loading" color="primary" type="submit">Open</q-btn>
            <q-btn padding="sm lg" :disabled="loading">Reset</q-btn>
          </q-btn-group>
        </q-card-actions>
      </q-card>
    </q-form>

  </q-page>

</template>

<script setup lang="ts">

import {ref} from 'vue'

import {useDatabasesStore} from '@/stores/databases'
import {useViewStore} from '@/stores/view'
import {useTemplateRef} from 'vue'
import {QInput} from 'quasar'

const databasesStore = useDatabasesStore()
const viewStore = useViewStore()

const password = ref("")
const keyFile = ref(null)

const loading = ref(false)
const showPassword = ref(false)

const errorMessage = ref("")

const $password = useTemplateRef<QInput>("$password")

setTimeout(() => {
  if (!$password.value) {return }
  $password.value.focus()
}, 100)

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
  if (typeof viewStore.current.database === 'undefined') {return }

  errorMessage.value = ""
  loading.value = true

  try {
    const keyFileBuffer = keyFile.value ? await readFile(keyFile.value) : null

    await databasesStore.unlockDatabase(viewStore.current.database, password.value, keyFileBuffer)

  } catch (e) {
    if (typeof e === 'string') {
      errorMessage.value = e
    } else if (e instanceof Error) {
      errorMessage.value = e.message
    }
  }

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
</style>
