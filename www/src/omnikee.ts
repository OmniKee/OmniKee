
interface OmniKee {
  greet(name: string): Promise<string>;

  increment(): Promise<void>;
  decrement(): Promise<void>;
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
  }


} else {
  console.log("OmniKee is in tauri mode")

  const {invoke} = await import('@tauri-apps/api/core')

  handle = {
    async greet(name) {return await invoke('greet', {name})},
    async increment() {return await invoke('increment')},
    async decrement() {return await invoke('decrement')},
  }

}

export default handle
