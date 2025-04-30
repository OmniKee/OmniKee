
import {type DatabaseOverview, type Entry} from 'omnikee-wasm'


export interface OmniKee {
  listDatabases(): Promise<DatabaseOverview[]>;
  loadDatabase(data: Uint8Array, password: string | null, keyfile: Uint8Array | null): Promise<DatabaseOverview>;
  closeDatabase(databaseIdx: number): Promise<void>,

  listEntries(databaseIdx: number, groupUuid: string): Promise<Entry[]>,
  revealProtected(databaseIdx: number, entryUuid: string, fieldName: string): Promise<string | undefined>,

  openExternalLink(url: string): Promise<void>,
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
    closeDatabase(databaseIdx) {return Promise.resolve(state.close_database(databaseIdx))},

    listEntries(databaseIdx, groupUuid) {return Promise.resolve(state.list_entries(databaseIdx, groupUuid))},
    revealProtected(databaseIdx, entryUuid, fieldName) {return Promise.resolve(state.reveal_protected(databaseIdx, entryUuid, fieldName))},

    openExternalLink(url) {
      window.open(url)
      return Promise.resolve()
    },
  }

} else {
  console.log("OmniKee will dispatch commands to Tauri backend")

  const {invoke} = await import('@tauri-apps/api/core')
  const {openUrl} = await import('@tauri-apps/plugin-opener')

  handle = {
    async listDatabases() {return await invoke('list_databases')},
    async loadDatabase(data, password, keyfile) {return await invoke('load_database', {data, password, keyfile})},
    async closeDatabase(databaseIdx) {return await invoke('close_database', {databaseIdx})},

    async listEntries(databaseIdx, groupUuid) {return await invoke<Entry[]>('list_entries', {databaseIdx, groupUuid})},
    async revealProtected(databaseIdx, entryUuid, fieldName) {
      return await invoke<string | undefined>("reveal_protected", {databaseIdx, entryUuid, fieldName})
    },

    async openExternalLink(url) {
      await openUrl(url)
    }

  }

}

export default handle
