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
    if (typeof current.value.database === 'undefined' || !current.value.group) {return undefined}
    return await ok.listEntries(current.value.database, current.value.group)
  }, undefined, loadingGroupEntries)


  return {
    current,
    database,
    groupEntries, loadingGroupEntries,
  }
})
