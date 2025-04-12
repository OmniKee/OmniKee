import {defineStore} from '#q-app/wrappers'
import {createPinia} from 'pinia'

export default defineStore((/* { ssrContext } */) => {
  const pinia = createPinia()

  // You can add Pinia plugins here
  // pinia.use(SomePiniaPlugin)

  return pinia
})
