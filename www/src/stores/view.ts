import {defineStore} from "pinia"
import {computed, ref, watch} from "vue"
import {asyncComputed} from "@vueuse/core"

import {useDatabasesStore} from "@/stores/databases"
import {type DatabaseOverview, type Entry} from "omnikee-wasm"
import ok from '@/omnikee'
import {useRouter} from "vue-router"


export type ViewState = {
  database: number | undefined,
  group: string | null,
  entry: string | null,
}

export const useViewStore = defineStore('view', () => {

  const databasesStore = useDatabasesStore()
  const router = useRouter()

  const current = ref<ViewState>({
    database: undefined,
    group: null,
    entry: null,
  })

  // update this to force reactivity in various places
  const counter = ref(0)

  watch(current.value, async () => {
    if (typeof current.value.database === "undefined") {return }
    if (current.value.database >= databasesStore.databases.length) {
      await router.push({name: '/'})
    }
  })

  const database = computed<DatabaseOverview | undefined>(() => {
    if (typeof current.value.database === 'undefined') {return undefined}
    return databasesStore.databases[current.value.database]
  })

  const loadingGroupEntries = ref(false)
  const groupEntries = asyncComputed<Entry[] | undefined>(async () => {
    if (counter.value) { /* force reactivity */}
    if (typeof current.value.database === 'undefined' || typeof database.value === 'undefined' || database.value.state !== 'Unlocked') {return undefined}
    const group = current.value.group || database.value?.root.uuid
    return await ok.listEntries(current.value.database, group)
  }, undefined, loadingGroupEntries)


  const entry = computed<Entry | undefined>(() => {
    if (counter.value) { /* force reactivity */}
    if (typeof current.value.database === 'undefined' || !current.value.entry || !groupEntries.value) {return undefined}
    return groupEntries.value.find(e => e.uuid === current.value.entry)
  })

  return {
    current,
    database,
    groupEntries, loadingGroupEntries,
    entry,
    counter,
  }
})
