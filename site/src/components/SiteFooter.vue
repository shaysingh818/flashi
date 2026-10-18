<template>
  <footer class="site-footer">
    <div class="container footer-inner">
      <div class="brand">
        <img :src="logoUrl" alt="" width="24" height="24" />
        <span>Flashi</span>
      </div>
      <div class="links">
        <a :href="REPO_URL" target="_blank" rel="noopener">GitHub</a>
        <a :href="`${REPO_URL}/issues`" target="_blank" rel="noopener">Report an issue</a>
        <a :href="RELEASES_URL" target="_blank" rel="noopener">Releases</a>
      </div>
      <p class="credit">
        UI built with <a href="https://shaysingh818.github.io/graphyne-web-component-library/" target="_blank" rel="noopener">Graphyne</a>.
      </p>
    </div>
  </footer>
</template>

<script setup lang="ts">
import { RELEASES_URL, REPO_URL } from "../config";

const logoUrl = `${import.meta.env.BASE_URL}flashi.svg`;
</script>

<style scoped>
.site-footer {
  padding: 32px 0;
  border-top: 1px solid var(--app-color-border);
  background-color: var(--app-color-surface);
  color: var(--app-color-text-muted);
  font-size: 0.9rem;
}

.footer-inner {
  display: flex;
  flex-wrap: wrap;
  align-items: center;
  justify-content: space-between;
  gap: 16px;
}

.brand {
  display: flex;
  align-items: center;
  gap: 8px;
  color: var(--app-color-text);
  font-weight: 600;
}

.links {
  display: flex;
  flex-wrap: wrap;
  gap: 20px;
}

a:hover {
  color: var(--app-color-accent);
}

.credit {
  margin: 0;
  width: 100%;
}

.credit a {
  color: var(--app-color-accent);
}
</style>
