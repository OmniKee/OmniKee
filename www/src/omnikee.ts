
import {type OTPResponse, type DatabaseOverview, type Entry, type ValueSet} from 'omnikee-wasm'

import {saveAs} from 'file-saver'


export interface OmniKee {
  listDatabases(): Promise<DatabaseOverview[]>,

  loadDemo(): Promise<DatabaseOverview>,
  loadDatabase(): Promise<DatabaseOverview>,
  saveDatabase(databaseIdx: number): Promise<void>,
  saveDatabaseAs(databaseIdx: number): Promise<void>,

  unlockDatabase(databaseIdx: number, password: string | null, keyfile: Uint8Array | null): Promise<DatabaseOverview>,
  lockDatabase(databaseIdx: number): Promise<DatabaseOverview>,

  closeDatabase(databaseIdx: number): Promise<void>,

  listEntries(databaseIdx: number, groupUuid: string): Promise<Entry[]>,
  revealProtected(databaseIdx: number, entryUuid: string, fieldName: string): Promise<string | undefined>,
  getOtp(databaseIdx: number, entryUuid: string, time: bigint): Promise<OTPResponse>,

  setGroupName(databaseIdx: number, groupUuid: string, name: string): Promise<void>,
  setField(databaseIdx: number, entryUuid: string, fieldName: string, value: ValueSet): Promise<void>,

  openExternalLink(url: string): Promise<void>,
  setWindowTitle(title: string): Promise<void>,
}

let handle: OmniKee

console.log(`OmniKee platform: ${process.env.TAURI_ENV_PLATFORM}`)

if (process.env.TAURI_ENV_PLATFORM === 'web') {
  console.log("OmniKee will use baked-in WebAssembly module")

  const ok = await import("omnikee-wasm")
  await ok.default()

  const state = ok.AppState.new()


  function promptFileOpen(): Promise<File> {
    return new Promise((resolve, reject) => {

      const $input = document.createElement('input')
      $input.type = 'file'

      if (!$input) {throw Error("could not create input")}

      $input.addEventListener('change', (event) => {
        const files = (event.target as HTMLInputElement).files

        if (files && files[0]) {
          resolve(files[0])
        } else {
          reject(new Error("no file selected"))
        }
      })

      $input.click();
    })
  }

  handle = {
    listDatabases() {return Promise.resolve(state.list_databases())},

    loadDemo() {return Promise.resolve(state.load_demo())},

    async loadDatabase() {
      const file = await promptFileOpen()
      const data = new Uint8Array(await file.arrayBuffer())

      return Promise.resolve(state.load_database_buffer(file.name, data))
    },

    saveDatabase(databaseIdx) {
      const desc = state.list_databases()[databaseIdx]
      if (!desc) {return Promise.reject(new Error("No database with that index"))}

      const data = state.save_database(databaseIdx)
      if (!data) {return Promise.reject(new Error("Did not get data from the backend"))}

      saveAs(new Blob([data], {type: "application/x-keepass"}), desc.file_name)

      return Promise.resolve()
    },

    async saveDatabaseAs(databaseIdx) {
      await this.saveDatabase(databaseIdx)
    },

    unlockDatabase(databaseIdx, password, keyfile) {return Promise.resolve(state.unlock_database(databaseIdx, password, keyfile))},
    lockDatabase(databaseIdx) {return Promise.resolve(state.lock_database(databaseIdx))},

    closeDatabase(databaseIdx) {return Promise.resolve(state.close_database(databaseIdx))},

    listEntries(databaseIdx, groupUuid) {return Promise.resolve(state.list_entries(databaseIdx, groupUuid))},
    revealProtected(databaseIdx, entryUuid, fieldName) {return Promise.resolve(state.reveal_protected(databaseIdx, entryUuid, fieldName))},
    getOtp(databaseIdx, entryUuid, time) {return Promise.resolve(state.get_otp(databaseIdx, entryUuid, time))},

    setGroupName(databaseIdx, groupUuid, name) {return Promise.resolve(state.set_group_name(databaseIdx, groupUuid, name))},
    setField(databaseIdx, entryUuid, fieldName, value) {return Promise.resolve(state.set_field(databaseIdx, entryUuid, fieldName, value))},

    openExternalLink(url) {
      window.open(url)
      return Promise.resolve()
    },

    setWindowTitle(title) {
      document.title = title
      return Promise.resolve()
    }
  }

} else {
  console.log("OmniKee will dispatch commands to Tauri backend")

  const {invoke} = await import('@tauri-apps/api/core')

  const platform = process.env.TAURI_ENV_PLATFORM || ''
  const isMobile = platform.includes("android") || platform.includes("ios")

  console.log(`Tauri on mobile: ${isMobile}`)

  handle = {
    async listDatabases() {return await invoke('list_databases')},

    async loadDemo() {return await invoke('load_demo')},

    // file picking is done from the Tauri backend
    async loadDatabase() {return await invoke('load_database')},
    async saveDatabase(databaseIdx) {return await invoke('save_database', {databaseIdx})},
    async saveDatabaseAs(databaseIdx) {return await invoke('save_database_as', {databaseIdx})},

    async unlockDatabase(databaseIdx, password, keyfile) {return await invoke('unlock_database', {databaseIdx, password, keyfile})},
    async lockDatabase(databaseIdx) {return await invoke('lock_database', {databaseIdx})},
    async closeDatabase(databaseIdx) {return await invoke('close_database', {databaseIdx})},

    async listEntries(databaseIdx, groupUuid) {return await invoke<Entry[]>('list_entries', {databaseIdx, groupUuid})},
    async revealProtected(databaseIdx, entryUuid, fieldName) {
      return await invoke<string | undefined>("reveal_protected", {databaseIdx, entryUuid, fieldName})
    },
    async getOtp(databaseIdx, entryUuid, time) {return await invoke("get_otp", {databaseIdx, entryUuid, time: Number(time)})},

    async setGroupName(databaseIdx, groupUuid, name) {return await invoke("set_group_name", {databaseIdx, groupUuid, name})},
    async setField(databaseIdx, entryUuid, fieldName, value) {return await invoke("set_field", {databaseIdx, entryUuid, fieldName, value})},

    async openExternalLink(url) {
      const {openUrl} = await import('@tauri-apps/plugin-opener')
      await openUrl(url)
    },

    async setWindowTitle(title) {
      if (isMobile) {return }
      const {getCurrentWindow} = await import('@tauri-apps/api/window')

      await getCurrentWindow().setTitle(title)
    }
  }

}

export default handle
