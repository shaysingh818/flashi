<template>
  <section id="features" class="section features">
    <div class="container">
      <h2 class="section-title">Everything you need to <span class="accent">study smarter</span></h2>
      <p class="section-subtitle">
        Flashi keeps your study material, your sessions and your results in one place, on your own machine.
      </p>

      <div class="grid">
        <GnCard v-for="feature in features" :key="feature.title">
          <div class="icon"><font-awesome-icon :icon="feature.icon" /></div>
          <h3>{{ feature.title }}</h3>
          <p>{{ feature.description }}</p>
        </GnCard>
      </div>
    </div>
  </section>
</template>

<script setup lang="ts">
import { GnCard } from "@graphyne/web-components";

const features = [
  {
    icon: "fa-regular fa-clone",
    title: "Subjects & decks",
    description: "Group questions into decks and decks into subjects, so every topic you study has its own home.",
  },
  {
    icon: "fa-regular fa-calendar",
    title: "Study sessions",
    description: "Pull decks from any subject into a session and work through the questions one by one.",
  },
  {
    icon: "fa-regular fa-chart-bar",
    title: "Metrics that matter",
    description: "Track accuracy per session and per question to find out where you need more practice.",
  },
  {
    icon: "fa-solid fa-square-root-variable",
    title: "Markdown, math & code",
    description: "Write questions in Markdown with KaTeX formulas and syntax-highlighted code blocks.",
  },
  {
    icon: "fa-solid fa-file-export",
    title: "Import & export decks",
    description: "Share decks with classmates or back them up, and import decks others have made.",
  },
  {
    icon: "fa-solid fa-lock",
    title: "Local first",
    description: "Your data is stored in a local database on your computer. No account or internet connection needed.",
  },
];
</script>

<style scoped>
.features {
  background-color: var(--app-color-surface);
}

.grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(280px, 1fr));
  gap: 20px;
}

/* GnCard is light-themed with no color props, so restyle it for the dark palette. */
.grid :deep(.gn-card) {
  height: 100%;
  background-color: var(--app-color-background);
  border-color: var(--app-color-border);
  border-radius: 12px;
}

.grid :deep(.gn-card__body) {
  padding: 24px;
}

.icon {
  display: grid;
  place-items: center;
  width: 44px;
  height: 44px;
  margin-bottom: 16px;
  border-radius: 10px;
  font-size: 1.15rem;
  color: var(--app-color-accent);
  background-color: color-mix(in srgb, var(--app-color-accent) 14%, transparent);
}

.grid h3 {
  margin: 0 0 6px;
  font-size: 1.1rem;
}

.grid p {
  margin: 0;
  opacity: 0.75;
}
</style>
