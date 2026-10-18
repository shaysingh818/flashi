<template>
  <section id="top" class="hero">
    <div class="container hero-inner">
      <div class="hero-copy">
        <GnTag label="Free & open source desktop app" variant="secondary" />
        <h1>
          Remember more with <span class="accent">active recall</span>.
        </h1>
        <p class="lead">
          Flashi helps you organise what you're learning into subjects and decks,
          quiz yourself in focused study sessions, and see exactly which questions
          you keep missing. It runs locally on Windows, macOS and Linux.
        </p>
        <div class="actions">
          <GnButton variant="primary" @gn-click="scrollTo('#download')">
            <font-awesome-icon icon="fa-solid fa-download" />
            Download Flashi
          </GnButton>
          <GnButton variant="secondary" @gn-click="openRepo">
            <font-awesome-icon icon="fa-brands fa-github" />
            View on GitHub
          </GnButton>
        </div>
      </div>

      <AppPreview class="hero-preview" />
    </div>
  </section>
</template>

<script setup lang="ts">
import { GnButton, GnTag } from "@graphyne/web-components";
import AppPreview from "./AppPreview.vue";
import { REPO_URL } from "../config";

function scrollTo(selector: string) {
  document.querySelector(selector)?.scrollIntoView({ behavior: "smooth" });
}

function openRepo() {
  window.open(REPO_URL, "_blank", "noopener");
}
</script>

<style scoped>
.hero {
  padding: 72px 0 80px;
  background:
    radial-gradient(60% 60% at 80% 20%, color-mix(in srgb, var(--app-color-accent) 18%, transparent), transparent 70%),
    var(--app-color-background);
}

.hero-inner {
  display: grid;
  grid-template-columns: 1fr 1fr;
  align-items: center;
  gap: 48px;
}

h1 {
  margin: 20px 0 16px;
  font-size: clamp(2.2rem, 5vw, 3.4rem);
  line-height: 1.1;
  font-weight: 800;
  letter-spacing: -0.02em;
}

.lead {
  margin: 0 0 32px;
  font-size: 1.1rem;
  color: var(--app-color-text-muted);
  max-width: 52ch;
}

.actions {
  display: flex;
  flex-wrap: wrap;
  gap: 12px;
}

.actions :deep(svg) {
  margin-right: 8px;
}

@media (max-width: 860px) {
  .hero-inner {
    grid-template-columns: 1fr;
  }
}
</style>
