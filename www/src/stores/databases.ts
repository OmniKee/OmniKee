import {defineStore} from 'pinia'
import {ref} from "vue"

import {type DatabaseOverview} from 'omnikee-wasm'

import ok from '@/omnikee'

import {useViewStore} from '@/stores/view'
import {useRouter} from 'vue-router'



export const useDatabasesStore = defineStore('databases', (/* { ssrContext } */) => {
  const databases = ref<DatabaseOverview[]>([])

  const viewStore = useViewStore()
  const router = useRouter()

  async function refresh() {
    const res = await ok.listDatabases()
    databases.value = res
  }

  async function loadDatabase(data: Uint8Array, password: string | null, keyFile: Uint8Array | null) {
    const res = await ok.loadDatabase(data, password, keyFile)
    databases.value.push(res)

    return res
  }

  async function closeDatabase(databaseIdx: number) {
    await ok.closeDatabase(databaseIdx)

    await refresh()

    // are we pointing to an invalid database index?
    if (
      typeof viewStore.current.database !== "undefined" &&     // there is a database selected
      viewStore.current.database >= databases.value.length     // that database is out of bounds
    ) {

      // the database index is invalid and needs fixing - see what we can do
      if (databases.value.length > 0) {
        // fix by going to the last valid entry
        await router.push({name: '/database/[i]/', params: {i: databases.value.length - 1}})
      } else {
        // fix by going to the start page
        await router.push({name: '/'})
      }
    }
  }

  return {databases, refresh, loadDatabase, closeDatabase}
})
