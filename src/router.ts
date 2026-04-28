import { createMemoryHistory, createRouter, RouteRecordRaw } from "vue-router";

import HomePage from "./pages/HomePage.vue";
import SubjectsPage from "./pages/subject/SubjectsPage.vue";
import CreateSubjects from "./pages/subject/CreateSubjects.vue";
import EditSubjects from "./pages/subject/EditSubjects.vue";
import SessionsPage from "./pages/SessionsPage.vue";
import SettingsPage from "./pages/SettingsPage.vue";
import SubjectsDetail from "./pages/subject/SubjectsDetail.vue";
import CreateDeck from "./pages/deck/CreateDeck.vue";
import EditDeck from "./pages/deck/EditDeck.vue";
import DeckDetail from "./pages/deck/DeckDetail.vue";
import CreateQuestion from "./pages/question/CreateQuestion.vue";

const routes: Array<RouteRecordRaw> = [
  // home
  { path: "/", component: HomePage },

  // subjects
  { path: "/subjects", component: SubjectsPage },
  { path: "/subjects/create", component: CreateSubjects },
  { path: "/subjects/edit", component: EditSubjects },
  { path: "/subjects/detail", component: SubjectsDetail },

  // decks
  { path: "/deck/create", component: CreateDeck },
  { path: "/deck/edit", component: EditDeck },
  { path: "/deck/detail", component: DeckDetail },
  
  // questions
  { path: "/question/create", component: CreateQuestion },

  
  // sessions
  { path: "/sessions", component: SessionsPage },

  // sessions
  { path: "/settings", component: SettingsPage },
];

const router = createRouter({
  history: createMemoryHistory(),
  routes,
});

export default router;
