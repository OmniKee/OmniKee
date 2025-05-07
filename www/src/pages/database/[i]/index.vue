<template>

  <DatabaseOverview v-if="viewStore.database?.state === 'Unlocked'" />
  <DatabaseUnlock v-if="viewStore.database?.state === 'Locked'" />

</template>


<script setup lang="ts">
import {watch} from 'vue'
import {useRoute} from 'vue-router'
import {useViewStore} from '@/stores/view'

definePage({
  meta: {
    title() {
      const viewStore = useViewStore()
      return viewStore.database?.name || viewStore.current.database
    },
  },
})

const route = useRoute('/database/[i]/')

const viewStore = useViewStore()

function updateRoute() {
  viewStore.current.database = +route.params.i
}

updateRoute()
watch(route, updateRoute)

</script>
