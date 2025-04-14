<template>
  <q-page class="row items-center justify-evenly">
    <p>
      <q-input v-model="name" label="Name" />
      <q-btn @click="onIncrement" label="+" />
      <q-btn @click="onDecrement" label="-" />

    </p>
    <p>{{ message }}</p>
  </q-page>
</template>

<script setup lang="ts">
import {ref} from 'vue'
import {computedAsync} from '@vueuse/core'
import omnikee from '@/omnikee'

const name = ref("World")
const dummy = ref(0)

const message = computedAsync(async () => {
  const _ = dummy.value  // eslint-disable-line @typescript-eslint/no-unused-vars
  return await omnikee.greet(name.value)
}
  , "Loading...")

async function onIncrement() {
  await omnikee.increment()
  forceRefresh()
}

async function onDecrement() {
  await omnikee.decrement()
  forceRefresh()
}

function forceRefresh() {
  dummy.value += 1
}


</script>
