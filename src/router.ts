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
import ViewQuestion from "./pages/question/ViewQuestion.vue";
import EditQuestion from "./pages/question/EditQuestion.vue";
import SessionDetail from "./pages/session/SessionDetail.vue";
import CreateSession from "./pages/session/CreateSession.vue";
import EditSession from "./pages/session/EditSession.vue";
import AddDeckToSession from "./pages/session/AddDeckToSession.vue";

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
  { path: "/question/view", component: ViewQuestion },
  { path: "/question/edit", component: EditQuestion },

  // sessions
  { path: "/sessions", component: SessionsPage },
  { path: "/sessions/detail", component: SessionDetail },
  { path: "/sessions/create", component: CreateSession },
  { path: "/sessions/edit", component: EditSession },
  { path: "/sessions/deck/add", component: AddDeckToSession },

  // sessions
  { path: "/settings", component: SettingsPage },
];

const router = createRouter({
  history: createMemoryHistory(),
  routes,
});

export default router;
