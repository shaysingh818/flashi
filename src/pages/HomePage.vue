<template>

  <div class="flex flex-col">
    <FormHeader
      header="Welcome to Flashi!"
      description="Flashi helps you learn anything with flashcards. Organize your material into subjects and decks, write questions in a few different formats, then review them in focused study sessions and watch your progress over time."
    />

    <div class="flex flex-row gap-4 p-4">
      <GnButton :disabled="false" variant="primary" @gn-click="goTo('/subjects/create')">
        Create your first subject
      </GnButton>
      <GnButton :disabled="false" variant="secondary" @gn-click="goTo('/sessions/create')">
        Start a session
      </GnButton>
    </div>

    <div class="flex items-left p-4">
      <h2> Your library </h2>
    </div>

    <div class="grid grid-cols-3 gap-4 place-center p-4">
      <GnMetricCard metric="Subjects" :numeric="subjectCount" />
      <GnMetricCard metric="Decks" :numeric="deckCount" />
      <GnMetricCard metric="Sessions" :numeric="sessionCount" />
    </div>

    <div class="flex items-left p-4">
      <h2> Getting started </h2>
    </div>

    <div class="flex flex-col p-4">
      <div class="flex flex-col section">
        <GnListTile
          v-for="(step, index) in steps"
          :key="index"
          :title="`${index + 1}. ${step.title}`"
          :description="step.description"
          color="var(--app-color-accent)"
          borderColor="var(--app-color-background)"
          textColor="var(--app-color-text)"
          descriptionColor="var(--app-color-text-muted)"
          trailingColor="var(--app-color-text-muted)"
          @gn-click="goTo(step.path)">
          <template #leading>
            <font-awesome-icon :icon="step.icon" />
          </template>
          <template #trailing>
            <font-awesome-icon icon="fa-solid fa-chevron-right" />
          </template>
        </GnListTile>
      </div>
    </div>

    <div class="flex items-left p-4">
      <h2> Question types </h2>
    </div>

    <div class="grid grid-cols-3 gap-4 p-4">
      <div
        v-for="(type, index) in questionTypes"
        :key="index"
        class="flex flex-col gap-2 p-6 section">
        <font-awesome-icon :icon="type.icon" class="type-icon" />
        <h3>{{ type.title }}</h3>
        <p>{{ type.description }}</p>
      </div>
    </div>

    <div class="flex items-left p-4">
      <h2> Tips </h2>
    </div>

    <div class="flex flex-col p-4">
      <div class="flex flex-col section">
        <GnListTile
          v-for="(tip, index) in tips"
          :key="index"
          :title="tip.title"
          :description="tip.description"
          color="var(--app-color-accent)"
          borderColor="var(--app-color-background)"
          textColor="var(--app-color-text)"
          descriptionColor="var(--app-color-text-muted)"
          :disabled="!tip.path"
          @gn-click="goTo(tip.path)">
          <template #leading>
            <font-awesome-icon :icon="tip.icon" />
          </template>
        </GnListTile>
      </div>
    </div>

  </div>
</template>


<script setup>
import { ref, onMounted } from "vue";
import { useRouter } from "vue-router";
import { GnButton, GnListTile } from "@graphyne/web-components";
import { analyticsStore } from "../api/Analytics";
import FormHeader from '../components/shared/forms/FormHeader.vue';

const router = useRouter();

const subjectCount = ref(0);
const deckCount = ref(0);
const sessionCount = ref(0);

const steps = [
  {
    title: "Create a subject",
    description: "Subjects are the top-level topics you're studying, like Biology, Rust or Spanish.",
    icon: "fa-solid fa-folder-plus",
    path: "/subjects/create",
  },
  {
    title: "Add decks to your subject",
    description: "Open a subject to group related questions into decks, or import a deck someone exported.",
    icon: "fa-regular fa-clone",
    path: "/subjects",
  },
  {
    title: "Write questions",
    description: "Open a deck and add questions using multiple choice, open ended or ordered sequence answers.",
    icon: "fa-solid fa-pen-to-square",
    path: "/subjects",
  },
  {
    title: "Build a study session",
    description: "Sessions pull decks from any of your subjects together so you can review them in one sitting.",
    icon: "fa-regular fa-calendar",
    path: "/sessions/create",
  },
  {
    title: "Review and track progress",
    description: "Hit play on a session to start reviewing, then check metrics to see your accuracy and review time.",
    icon: "fa-regular fa-chart-bar",
    path: "/metrics",
  },
];

const questionTypes = [
  {
    title: "Multiple choice",
    description: "Provide several options and mark which ones are correct.",
    icon: "fa-solid fa-list-check",
  },
  {
    title: "Open ended",
    description: "Write a free-form answer with keywords used to check your response.",
    icon: "fa-solid fa-align-left",
  },
  {
    title: "Ordered sequence",
    description: "Put a series of steps or items into the correct order.",
    icon: "fa-solid fa-arrow-down-1-9",
  },
];

const tips = [
  {
    title: "Format questions with Markdown",
    description: "Question text supports Markdown, syntax-highlighted code blocks and LaTeX math using $...$.",
    icon: "fa-brands fa-markdown",
  },
  {
    title: "Share decks with export and import",
    description: "Set an export folder in settings, then use a deck's menu to export it. Import decks from a subject's page.",
    icon: "fa-solid fa-file-export",
    path: "/settings",
  },
  {
    title: "Review a little every day",
    description: "Short, regular sessions beat long cramming sessions. The metrics page shows how consistent you've been.",
    icon: "fa-solid fa-fire",
    path: "/metrics",
  },
];

function goTo(path) {
  if (path) router.push({ path });
}

onMounted(async () => {
  [
    subjectCount.value,
    deckCount.value,
    sessionCount.value,
  ] = await analyticsStore.generalMetrics();
});
</script>

<style scoped>
h2 {
  margin-bottom: 0;
  font-weight: bold;
  color: var(--app-color-text);
}

h3 {
  margin: 0;
  font-size: 17px;
  font-weight: bold;
  color: var(--app-color-text);
}

p {
  margin: 0;
  font-size: 14px;
  color: var(--app-color-text-muted);
}

.section {
  background-color: var(--app-color-surface);
  border-radius: 10px;
  overflow: hidden;
}

.type-icon {
  color: var(--app-color-accent);
  width: 22px;
  height: 22px;
}
</style>
