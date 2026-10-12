<script setup lang="ts">
import {
  GnNavigationBar,
  GnNavigationItem,
  GnNavigationHeader
} from "@graphyne/web-components";
import { invoke } from "@tauri-apps/api/core";
import { onMounted, ref } from "vue";

const ready = ref(false);
const setupError = ref('');

async function runSetup() {
  setupError.value = '';
  try {
    await invoke('setup');
    ready.value = true;
  } catch (error) {
    console.error("Error running setup: ", error);
    setupError.value = String(error);
  }
}

onMounted(runSetup);

</script>

<template>
  <div class="app-shell">
  <!-- <NavigationBar /> -->
    <GnNavigationBar direction="col" backgroundColor="rgb(28 25 23)">
        <template #header>
            <GnNavigationHeader title="Flashi" titleSize="12px" direction="col">
                <font-awesome-icon icon="fa-regular fa-clone" />
            </GnNavigationHeader>
        </template>
        <template #items>
        
            <RouterLink to="/" >
                <GnNavigationItem label="home" direction="col">
                    <font-awesome-icon icon="fa-regular fa-home" />
                </GnNavigationItem>
            </RouterLink>
            
            <RouterLink to="/subjects" >
                <GnNavigationItem label="subjects" direction="col">
                    <font-awesome-icon icon="fa-regular fa-clone" />
                </GnNavigationItem>
            </RouterLink>
    
            <RouterLink to="/sessions" >
                <GnNavigationItem label="sessions" direction="col">
                    <font-awesome-icon icon="fa-regular fa-calendar" />
                </GnNavigationItem>
            </RouterLink>
    
            <RouterLink to="/metrics" >
                <GnNavigationItem label="metrics" direction="col">
                    <font-awesome-icon icon="fa-regular fa-chart-bar" />
                </GnNavigationItem>
            </RouterLink>
    
            <RouterLink to="/settings" >
                <GnNavigationItem label="settings" direction="col">           
                    <font-awesome-icon icon="fa-regular fa-sun" />
                </GnNavigationItem>
            </RouterLink>
            
        </template>
    </GnNavigationBar>

    <main class="content">
        <RouterView v-if="ready" />
        <div v-else-if="setupError" class="setup-status">
            <p>Setup failed: {{ setupError }}</p>
            <button @click="runSetup">Retry</button>
        </div>
        <div v-else class="setup-status">
            <p>Setting up Flashi...</p>
        </div>
    </main>
  </div>
</template>

<style scoped>
.app-shell {
  display: flex;
  height: 100vh;
  width: 100vw;
  overflow: hidden;
}

main {
    flex: 1;
    min-width: 0;
    overflow-y: auto;
}

.setup-status {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    gap: 12px;
    height: 100%;
    color: var(--app-color-text-muted);
}

.setup-status button {
    padding: 6px 16px;
    border-radius: 6px;
    background-color: var(--app-color-accent);
    color: var(--app-color-text);
    cursor: pointer;
}
</style>
