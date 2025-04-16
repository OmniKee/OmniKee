import {createRouter} from 'vue-router/auto'
import {routes} from 'vue-router/auto-routes'

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


export default router

