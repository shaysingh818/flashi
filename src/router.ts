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
import MetricsPage from "./pages/MetricsPage.vue";

const routes: Array<RouteRecordRaw> = [
  // home
  { path: "/", component: HomePage },

  // subjects
  { path: "/subjects", component: SubjectsPage },
  { path: "/subjects/create", component: CreateSubjects },
  { path: "/subjects/edit/:id", component: EditSubjects },
  { path: "/subjects/detail/:id", component: SubjectsDetail },

  // decks
  { path: "/deck/create/:id", component: CreateDeck },
  { path: "/deck/edit/:id", component: EditDeck },
  { path: "/deck/detail/:id", component: DeckDetail },

  // questions
  { path: "/question/create/:id", component: CreateQuestion },
  { path: "/question/view/:id", component: ViewQuestion },
  { path: "/question/edit/:id", component: EditQuestion },

  // sessions
  { path: "/sessions", component: SessionsPage },
  { path: "/sessions/create", component: CreateSession },
  { path: "/sessions/edit/:id", component: EditSession },
  { path: "/sessions/detail/:id", component: SessionDetail },
  { path: "/sessions/deck/add/:id", component: AddDeckToSession },

  // metrics
  { path: "/metrics", component: MetricsPage },

  // sessions
  { path: "/settings", component: SettingsPage },
];

const router = createRouter({
  history: createMemoryHistory(),
  routes,
});

export default router;
