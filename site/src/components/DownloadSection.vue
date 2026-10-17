<template>
  <section id="download" class="section download">
    <div class="container">
      <h2 class="section-title">Download <span class="accent">Flashi</span></h2>
      <p class="section-subtitle">
        <template v-if="version">Latest release: <strong>{{ version }}</strong>. </template>
        Pick your operating system below.
      </p>

      <div class="panel">
        <GnTabNavigationBar v-model="platform" backgroundColor="var(--app-color-surface)" textColor="var(--app-color-text)" v-slot="{ select }">
          <GnTabNavigationItem
            v-for="p in platforms"
            :key="p.id"
            :label="p.label"
            :selected="platform === p.id"
            @gn-click="select(p.id)">
            <font-awesome-icon :icon="p.icon" />
          </GnTabNavigationItem>
        </GnTabNavigationBar>

        <div class="panel-body">
          <p v-if="loading" class="status">Looking up the latest release…</p>

          <div v-else-if="options.length" class="options">
            <GnListTile
              v-for="option in options"
              :key="option.url"
              :title="option.label"
              :description="formatSize(option.size)"
              borderColor="var(--app-color-border)"
              textColor="var(--app-color-text)"
              descriptionColor="var(--app-color-text-muted)"
              @gn-click="download(option.url)">
              <template #leading>
                <font-awesome-icon :icon="currentPlatform.icon" />
              </template>
              <template #trailing>
                <font-awesome-icon icon="fa-solid fa-download" />
              </template>
            </GnListTile>
          </div>

          <div v-else class="status">
            <p>
              No {{ currentPlatform.label }} build is published yet.
              Check the releases page for the newest builds.
            </p>
          </div>

          <div class="footer-actions">
            <GnButton variant="ghost" @gn-click="openReleases">
              <font-awesome-icon icon="fa-brands fa-github" />
              All releases
            </GnButton>
          </div>
        </div>
      </div>
    </div>
  </section>
</template>

<script setup lang="ts">
import { computed, ref } from "vue";
import { GnButton, GnListTile, GnTabNavigationBar, GnTabNavigationItem } from "@graphyne/web-components";
import { detectPlatform, formatSize, useLatestRelease, type Platform } from "../composables/useLatestRelease";

const platforms: { id: Platform; label: string; icon: string }[] = [
  { id: "windows", label: "Windows", icon: "fa-brands fa-windows" },
  { id: "macos", label: "macOS", icon: "fa-brands fa-apple" },
  { id: "linux", label: "Linux", icon: "fa-brands fa-linux" },
];

const platform = ref<Platform>(detectPlatform() ?? "windows");
const { loading, version, releaseUrl, optionsFor } = useLatestRelease();

const currentPlatform = computed(() => platforms.find((p) => p.id === platform.value)!);
const options = computed(() => optionsFor(platform.value));

function download(url: string) {
  window.location.href = url;
}

function openReleases() {
  window.open(releaseUrl.value, "_blank", "noopener");
}
</script>

<style scoped>
.panel {
  border: 1px solid var(--app-color-border);
  border-radius: 12px;
  overflow: hidden;
  background-color: var(--app-color-surface);
}

.panel-body {
  padding: 16px;
}

.options {
  border: 1px solid var(--app-color-border);
  border-radius: 8px;
  overflow: hidden;
}

.status {
  margin: 0;
  padding: 24px 8px;
  color: var(--app-color-text-muted);
  text-align: center;
}

.status p {
  margin: 0;
}

.footer-actions {
  display: flex;
  justify-content: flex-end;
  margin-top: 16px;
}

.footer-actions :deep(svg) {
  margin-right: 8px;
}
</style>
