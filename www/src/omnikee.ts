let implGreet

console.log(process.env)

if (process.env.DEPLOYMENT_TYPE === 'web') {
  console.log("OmniKee is in web mode")

  const ok = await import("omnikee-wasm")
  await ok.default()

  implGreet = (name: string) => Promise.resolve(ok.greet(name))

} else {
  console.log("OmniKee is in tauri mode")

  const {invoke} = await import('@tauri-apps/api/core')

  implGreet = async (name: string) => await invoke('greet', {name})

}

export const greet = implGreet
