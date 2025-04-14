<template>
  <q-page class="row items-center justify-evenly">
    <p>
      <q-input v-model="name" label="Name" />
      <q-btn @click="omnikee.increment" label="+" />
      <q-btn @click="omnikee.decrement" label="-" />

    </p>
    <p>{{ message }}</p>
  </q-page>
</template>

<script setup lang="ts">
import {ref} from 'vue'
import {computedAsync} from '@vueuse/core'
import omnikee from '@/omnikee'

const name = ref("World")

const message = computedAsync(async () => await omnikee.greet(name.value), "Loading...")

</script>
