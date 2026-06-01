<template>
  <div class="flex col-center w-100">
    <br />
    <SearchComponent text="Search Sessions" @search="search" />
    <div class="flex items-center justify-center">
      <h2>Sessions</h2>
    </div>
    <SessionListView />
    <HoverButton icon="fa-solid fa-plus" @click="createSession" />
  </div>
</template>

<script setup>
import { ref } from "vue";
import { useRouter } from "vue-router";
import { sessionStore } from "../api";
import SearchComponent from "../components/shared/forms/SearchComponent.vue";
import SessionListView from "../components/session/SessionListView.vue";
import HoverButton from "../components/shared/buttons/HoverButton.vue";


const router = useRouter();

async function search(searchTerm) {
  await sessionStore.searchSessions(searchTerm);
}

async function createSession() {
  router.push({ path: "/sessions/create" });
}

</script>

<style scoped>
h2 {
  color: white;
}
</style>
