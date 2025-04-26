
import {type DatabaseOverview, type Entry} from 'omnikee-wasm'


export interface OmniKee {
  listDatabases(): Promise<DatabaseOverview[]>;
  loadDatabase(data: Uint8Array, password: string | null, keyfile: Uint8Array | null): Promise<DatabaseOverview>;

  listEntries(database_idx: number, group_uuid: string): Promise<Entry[]>,
  revealProtected(database_idx: number, entry_uuid: string, field_name: string): Promise<string | undefined>,
}

let handle: OmniKee

console.log(`OmniKee platform: ${process.env.TAURI_ENV_PLATFORM}`)

if (process.env.TAURI_ENV_PLATFORM === 'web') {
  console.log("OmniKee will use baked-in WebAssembly module")

  const ok = await import("omnikee-wasm")
  await ok.default()

  const state = ok.AppState.new()

  handle = {
    listDatabases() {return Promise.resolve(state.list_databases())},
    loadDatabase(data, password, keyfile) {return Promise.resolve(state.load_database(data, password, keyfile))},

    listEntries(database_idx, group_uuid) {return Promise.resolve(state.list_entries(database_idx, group_uuid))},
    revealProtected(database_idx, entry_uuid, field_name) {return Promise.resolve(state.reveal_protected(database_idx, entry_uuid, field_name))},
  }

} else {
  console.log("OmniKee will dispatch commands to Tauri backend")

  const {invoke} = await import('@tauri-apps/api/core')

  handle = {
    async listDatabases() {return await invoke('list_databases')},
    async loadDatabase(data, password, keyfile) {return await invoke('load_database', {data, password, keyfile})},

    async listEntries(databaseIdx, groupUuid) {return await invoke<Entry[]>('list_entries', {databaseIdx, groupUuid})},
    async revealProtected(databaseIdx, entryUuid, fieldName) {
      return await invoke<string | undefined>("reveal_protected", {databaseIdx, entryUuid, fieldName})
    },

  }

}

export default handle
