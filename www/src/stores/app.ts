import {defineStore} from "pinia"

export const useAppStore = defineStore('app', () => {

  const platform = process.env.TAURI_ENV_PLATFORM

  const web = platform === 'web'
  const tauri = !web;

  const mac = platform === 'darwin'
  const windows = platform === 'windows'
  const linux = platform === 'linux'

  const desktop = mac || windows || linux

  const android = platform === 'android'
  const ios = platform === 'ios'

  const mobile = android || ios


  const is = {
    web,
    tauri,

    mac,
    windows,
    linux,
    desktop,

    android,
    ios,
    mobile,
  }

  return {platform, is}
})
