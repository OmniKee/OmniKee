import {defineStore} from 'pinia'
import {ref} from "vue"

import {type DatabaseOverview} from 'omnikee-wasm'

import ok from '@/omnikee'



export const useDatabasesStore = defineStore('databases', (/* { ssrContext } */) => {
  const databases = ref<DatabaseOverview[]>([])

  async function refresh() {
    const res = await ok.listDatabases()
    databases.value = res
  }

  async function loadDatabase(data: Uint8Array, password: string | null, keyFile: Uint8Array | null) {
    const res = await ok.loadDatabase(data, password, keyFile)
    databases.value.push(res)

    return res
  }

  return {databases, refresh, loadDatabase}
})
