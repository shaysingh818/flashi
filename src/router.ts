import { createMemoryHistory, createRouter, RouteRecordRaw } from 'vue-router'

import HomePage from './pages/HomePage.vue'
import SubjectsPage from './pages/subject/SubjectsPage.vue'
import CreateSubjects from './pages/subject/CreateSubjects.vue'
import SessionsPage from './pages/SessionsPage.vue'
import SettingsPage from './pages/SettingsPage.vue'

const routes: Array<RouteRecordRaw> =  [

  // home
  { path: '/', component: HomePage },

  // subjects
  { path: '/subjects', component: SubjectsPage },
  { path: '/subjects/create', component: CreateSubjects },

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
