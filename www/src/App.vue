<template>
  <router-view />
</template>

<script setup lang="ts">

import {greet} from './omnikee'

async function welcome() {
  alert(await greet('world'))
}

welcome().catch(console.error)
//
</script>
