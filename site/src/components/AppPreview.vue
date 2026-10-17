<!-- A small, non-interactive mock of the desktop app built from the same Graphyne components it uses. -->
<template>
  <div class="window" aria-hidden="true">
    <div class="titlebar">
      <span class="dot" /><span class="dot" /><span class="dot" />
      <span class="window-title">flashi</span>
    </div>

    <div class="window-body">
      <GnNavigationBar direction="col" backgroundColor="var(--app-color-surface)">
        <template #header>
          <GnNavigationHeader title="Flashi" titleSize="11px" direction="col">
            <font-awesome-icon icon="fa-regular fa-clone" />
          </GnNavigationHeader>
        </template>
        <template #items>
          <GnNavigationItem label="home" direction="col"><font-awesome-icon icon="fa-solid fa-house" /></GnNavigationItem>
          <GnNavigationItem label="subjects" direction="col"><font-awesome-icon icon="fa-regular fa-clone" /></GnNavigationItem>
          <GnNavigationItem label="sessions" direction="col"><font-awesome-icon icon="fa-regular fa-calendar" /></GnNavigationItem>
          <GnNavigationItem label="metrics" direction="col"><font-awesome-icon icon="fa-regular fa-chart-bar" /></GnNavigationItem>
        </template>
      </GnNavigationBar>

      <div class="window-content">
        <div class="metrics">
          <GnMetricCard metric="Average accuracy" :numeric="86" backgroundColor="var(--app-color-surface)" labelColor="var(--app-color-text-muted)" valueColor="var(--app-color-text)" />
          <GnMetricCard metric="Questions reviewed" :numeric="142" backgroundColor="var(--app-color-surface)" labelColor="var(--app-color-text-muted)" valueColor="var(--app-color-text)" />
        </div>

        <GnListTile
          v-for="deck in decks"
          :key="deck.title"
          :title="deck.title"
          :description="deck.description"
          borderColor="var(--app-color-background)"
          textColor="var(--app-color-text)"
          descriptionColor="var(--app-color-text-muted)"
          backgroundColor="var(--app-color-surface)">
          <template #leading>
            <font-awesome-icon icon="fa-solid fa-list-check" />
          </template>
          <template #tags>
            <GnTag v-for="tag in deck.tags" :key="tag" :label="tag" />
          </template>
        </GnListTile>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { GnListTile, GnMetricCard, GnNavigationBar, GnNavigationHeader, GnNavigationItem, GnTag } from "@graphyne/web-components";

const decks = [
  { title: "Organic Chemistry", description: "48 questions", tags: ["chemistry"] },
  { title: "Linear Algebra", description: "32 questions", tags: ["math"] },
  { title: "Spanish Verbs", description: "120 questions", tags: ["language"] },
];
</script>

<style scoped>
.window {
  border: 1px solid var(--app-color-border);
  border-radius: 12px;
  overflow: hidden;
  background-color: var(--app-color-background);
  box-shadow: 0 30px 60px -20px rgb(0 0 0 / 0.6);
  pointer-events: none;
  user-select: none;
}

.titlebar {
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 10px 14px;
  background-color: var(--app-color-surface);
  border-bottom: 1px solid var(--app-color-border);
}

.dot {
  width: 10px;
  height: 10px;
  border-radius: 50%;
  background-color: var(--app-color-border);
}

.window-title {
  margin-left: 8px;
  font-size: 12px;
  color: var(--app-color-text-muted);
}

.window-body {
  display: flex;
  min-height: 340px;
}

/* GnNavigationBar sets height: 100%, which can't resolve here and blocks flex stretch. */
.window-body :deep(.gn-navigation-bar) {
  height: auto;
}

.window-content {
  flex: 1;
  min-width: 0;
  display: flex;
  flex-direction: column;
  gap: 8px;
  padding: 12px;
}

.metrics {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 8px;
  margin-bottom: 4px;
}
</style>
