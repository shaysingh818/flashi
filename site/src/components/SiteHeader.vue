<template>
  <header class="site-header">
    <div class="container header-inner">
      <a href="#top" class="brand">
        <GnNavigationHeader title="Flashi" direction="row" titleSize="18px" iconSize="1.5rem" textColor="var(--app-color-text)">
          <font-awesome-icon icon="fa-regular fa-clone" />
        </GnNavigationHeader>
      </a>

      <GnNavigationBar direction="row" borderColor="transparent">
        <template #items>
          <a v-for="link in links" :key="link.href" :href="link.href" :target="link.external ? '_blank' : undefined" rel="noopener" class="nav-link">
            <GnNavigationItem :label="link.label" direction="row">
              <font-awesome-icon :icon="link.icon" />
            </GnNavigationItem>
          </a>
        </template>
      </GnNavigationBar>
    </div>
  </header>
</template>

<script setup lang="ts">
import { GnNavigationBar, GnNavigationHeader, GnNavigationItem } from "@graphyne/web-components";
import { REPO_URL } from "../config";

const links = [
  { label: "features", href: "#features", icon: "fa-regular fa-star", external: false },
  { label: "download", href: "#download", icon: "fa-solid fa-download", external: false },
  { label: "github", href: REPO_URL, icon: "fa-brands fa-github", external: true },
];
</script>

<style scoped>
.site-header {
  position: sticky;
  top: 0;
  z-index: 10;
  background-color: color-mix(in srgb, var(--app-color-surface) 92%, transparent);
  backdrop-filter: blur(8px);
  border-bottom: 1px solid var(--app-color-border);
}

.header-inner {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 16px;
  min-height: 64px;
}

.nav-link :deep(*) {
  cursor: pointer;
}

/* Icons alone are enough on narrow screens. */
@media (max-width: 560px) {
  .nav-link :deep(span) {
    display: none;
  }
}
</style>
