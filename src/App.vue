<script setup lang="ts">
import { GnNavigationBar, GnNavigationItem, GnNavigationHeader } from "@graphyne/web-components";

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
    <RouterView />
  </main>
  </div>
</template>

<style scoped>
.app-shell {
  display: flex;
  height: 100vh;
}

main {
  overflow-y: auto;
}
</style>
