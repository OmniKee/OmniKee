
import {type DatabaseOverview} from 'omnikee-wasm'


export interface OmniKee {
  greet(name: string): Promise<string>;

  increment(): Promise<void>;
  decrement(): Promise<void>;

  listDatabases(): Promise<DatabaseOverview[]>;
  loadDatabase(data: Uint8Array, password: string | null, keyfile: Uint8Array | null): Promise<DatabaseOverview>;
}

let handle: OmniKee

if (process.env.DEPLOYMENT_TYPE === 'web') {
  console.log("OmniKee is in web mode")

  const ok = await import("omnikee-wasm")
  await ok.default()

  const state = ok.AppState.new()

  handle = {
    greet(name) {return Promise.resolve(state.greet(name))},
    increment() {return Promise.resolve(state.increment())},
    decrement() {return Promise.resolve(state.decrement())},

    listDatabases() {return Promise.resolve(state.list_databases())},
    loadDatabase(data, password, keyfile) {return Promise.resolve(state.load_database(data, password, keyfile))},
  }

} else {
  console.log("OmniKee is in tauri mode")

  const {invoke} = await import('@tauri-apps/api/core')

  handle = {
    async greet(name) {return await invoke('greet', {name})},
    async increment() {return await invoke('increment')},
    async decrement() {return await invoke('decrement')},

    async listDatabases() {return await invoke('list_databases')},
    async loadDatabase(data, password, keyfile) {return await invoke('load_database', {data, password, keyfile})},
  }

}

export default handle
