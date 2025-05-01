import {createRouter} from 'vue-router/auto'
import {routes} from 'vue-router/auto-routes'

import ok from '@/omnikee'

import {
  createMemoryHistory,
  createWebHashHistory,
  createWebHistory,
} from 'vue-router';

const createHistory = process.env.SERVER
  ? createMemoryHistory
  : (process.env.VUE_ROUTER_MODE === 'history' ? createWebHistory : createWebHashHistory);

const router = createRouter({
  history: createHistory(process.env.VUE_ROUTER_BASE),
  routes,
})


type TitleFunction = () => string

declare module 'vue-router/auto' {
  interface RouteMeta {
    title?: string | TitleFunction
  }
}

router.beforeEach(async (to) => {
  if (to.meta.title) {
    if (typeof to.meta.title === 'function') {
      setTimeout(() => {
        if (typeof to.meta.title === 'function') {
          const title = to.meta.title()
          void ok.setWindowTitle(`${title} | OmniKee`)
        }
      }, 100)
    } else if (typeof to.meta.title === 'string') {
      await ok.setWindowTitle(`${to.meta.title} | OmniKee`)
    } else {
      await ok.setWindowTitle('OmniKee')
    }
  } else {
    await ok.setWindowTitle('OmniKee')
  }
})

export default router

