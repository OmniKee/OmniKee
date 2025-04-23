
import {type DatabaseOverview, type Entry} from 'omnikee-wasm'


export interface OmniKee {
  listDatabases(): Promise<DatabaseOverview[]>;
  loadDatabase(data: Uint8Array, password: string | null, keyfile: Uint8Array | null): Promise<DatabaseOverview>;

  listEntries(database_idx: number, group_uuid: string): Promise<Entry[]>,
}

let handle: OmniKee

if (process.env.DEPLOYMENT_TYPE === 'web') {
  console.log("OmniKee is in web mode")

  const ok = await import("omnikee-wasm")
  await ok.default()

  const state = ok.AppState.new()

  handle = {
    listDatabases() {return Promise.resolve(state.list_databases())},
    loadDatabase(data, password, keyfile) {return Promise.resolve(state.load_database(data, password, keyfile))},

    listEntries(database_idx, group_uuid) {return Promise.resolve(state.list_entries(database_idx, group_uuid))},
  }

} else {
  console.log("OmniKee is in tauri mode")

  const {invoke} = await import('@tauri-apps/api/core')

  handle = {
    async listDatabases() {return await invoke('list_databases')},
    async loadDatabase(data, password, keyfile) {return await invoke('load_database', {data, password, keyfile})},

    async listEntries(databaseIdx, groupUuid) {return await invoke<Entry[]>('list_entries', {databaseIdx, groupUuid})},
  }

}

export default handle
