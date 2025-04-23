<template>
  <q-toolbar class="bg-accent">
    <q-btn flat icon="mdi-arrow-left-circle" :to="{name: '/database/[i]/', params: {i: route.params.i}}" />
    <q-toolbar-title class="text-subtitle2" shrink>{{ entry?.name || route.params.uuid }}</q-toolbar-title>
  </q-toolbar>

</template>

<script setup lang="ts">
import {computed} from 'vue'
import {useRoute} from 'vue-router'

import {useViewStore} from '@/stores/view'

const route = useRoute('/database/[i]/entry/[uuid]')

const viewStore = useViewStore()

viewStore.current.database = +route.params.i
viewStore.current.entry = route.params.uuid

const entry = computed(() => viewStore.entry)
</script>u
