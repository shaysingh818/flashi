<template>
  <section class="section">
    <div class="container">
      <h2 class="section-title">Questions that test <span class="accent">real recall</span></h2>
      <p class="section-subtitle">
        Go beyond flip cards. Mix question formats to practise the way you'll be tested.
      </p>

      <div class="list">
        <GnListTile
          v-for="type in questionTypes"
          :key="type.title"
          :title="type.title"
          :description="type.description"
          borderColor="var(--app-color-border)"
          textColor="var(--app-color-text)"
          descriptionColor="var(--app-color-text-muted)">
          <template #leading>
            <font-awesome-icon :icon="type.icon" />
          </template>
          <template #tags>
            <GnTag :label="type.tag" variant="secondary" />
          </template>
        </GnListTile>
      </div>
    </div>
  </section>
</template>

<script setup lang="ts">
import { GnListTile, GnTag } from "@graphyne/web-components";

const questionTypes = [
  {
    icon: "fa-solid fa-list-ul",
    title: "Multiple choice",
    description: "Pick the right answer out of several options.",
    tag: "recognition",
  },
  {
    icon: "fa-solid fa-pen",
    title: "Open ended",
    description: "Write your answer from memory, then compare it to the expected answer.",
    tag: "free recall",
  },
  {
    icon: "fa-solid fa-arrow-down-1-9",
    title: "Ordered sequence",
    description: "Put steps, events or stages in the correct order.",
    tag: "sequencing",
  },
];
</script>

<style scoped>
.list {
  border: 1px solid var(--app-color-border);
  border-radius: 12px;
  overflow: hidden;
  background-color: var(--app-color-surface);
}
</style>
