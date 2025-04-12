<template>
  <router-view />
</template>

<script setup lang="ts">

import {greet} from './omnikee'

function welcome() {
  alert(greet('world'))
}

welcome()
//
</script>
