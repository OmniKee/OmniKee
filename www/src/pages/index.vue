<template>
  <q-page class="row items-center justify-evenly">

    <q-form style="width: 80%" @submit.prevent.stop="onSubmit">
      <q-card>
        <q-card-section>
          <div class="text-h6">Open Database</div>

          <q-file v-model="database" name="database" accept=".kdbx" label="Database" clearable filled class="q-mt-sm">
            <template #prepend>
              <q-icon name="mdi-file-account" />
            </template>
          </q-file>
          <q-input v-model="password" name="password" type="password" label="Password" clearable filled class="q-mt-sm">
            <template #prepend>
              <q-icon name="mdi-key-variant" />
            </template>
          </q-input>
          <q-file v-model="keyFile" name="keyFile" label="Key file" clearable filled class="q-mt-sm">
            <template #prepend>
              <q-icon name="mdi-file-key" />
            </template>

          </q-file>

        </q-card-section>

        <q-card-actions align="right">
          <q-btn color="primary" type="submit">Open</q-btn>
          <q-btn>Reset</q-btn>
        </q-card-actions>
      </q-card>
    </q-form>
  </q-page>
</template>

<script setup lang="ts">
import {ref} from 'vue'

import ok from '@/omnikee'

const database = ref(null)
const password = ref("")
const keyFile = ref(null)


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

  const databaseBuffer = await readFile(database.value)
  const keyFileBuffer = keyFile.value ? await readFile(keyFile.value) : null

  const db = ok.loadDatabase(databaseBuffer, password.value, keyFileBuffer)

  console.log(db)
}
</script>
