import {defineStore} from "pinia"

export const useAppStore = defineStore('app', () => {

  const web = process.env.TAURI_ENV_PLATFORM === 'web'
  const tauri = !web;

  const mac = process.env.TAURI_ENV_PLATFORM === 'darwin'
  const windows = process.env.TAURI_ENV_PLATFORM === 'windows'
  const linux = process.env.TAURI_ENV_PLATFORM === 'linux'


  const is = {
    web,
    tauri,

    mac,
    windows,
    linux,
  }

  return {is}
})
