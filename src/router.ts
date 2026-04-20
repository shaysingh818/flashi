import { createMemoryHistory, createRouter, RouteRecordRaw } from 'vue-router'

import HomePage from './pages/HomePage.vue'
import SubjectsPage from './pages/SessionsPage.vue'
import SessionsPage from './pages/SettingsPage.vue'
import SettingsPage from './pages/SubjectsPage.vue'

const routes: Array<RouteRecordRaw> =  [

  // home
  { path: '/home', component: HomePage },

  // subjects
  { path: '/subjects', component: SubjectsPage },

  // sessions
  { path: '/sessions', component: SessionsPage },

  // sessions
  { path: '/settings', component: SettingsPage },

]

const router = createRouter({
  history: createMemoryHistory(),
  routes,
})

export default router
