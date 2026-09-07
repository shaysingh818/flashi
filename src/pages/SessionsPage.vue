<template>
  <div class="flex flex-col">

    <div class="p-4"> 
        <GnSearchFormField 
          v-model="searchTerm"
          inputBorderColor="rgb(28 25 23)" 
          buttonBorderColor="var(--app-color-accent)"
          buttonBackgroundColor="var(--app-color-accent)" 
          buttonIconColor="var(--app-color-text)"
          focusColor="var(--app-color-accent)" 
          textColor="var(--app-color-accent)"
          label="Search Sessions" 
          placeholder="Search Sessions"
          backgroundColor="var(--app-color-surface)"
          @gn-search="search"
        />
    </div>

    
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
import { GnSearchFormField } from '@graphyne/web-components';
import SessionListView from "../components/session/SessionListView.vue";
import HoverButton from "../components/shared/buttons/HoverButton.vue";


const router = useRouter();
const searchTerm = ref('');


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
