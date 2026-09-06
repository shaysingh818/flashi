<template>
  <div class="flex col-center w-100">
    <br />
    <SearchComponent text="Search Sessions" @search="search" />

    <div class="w-85">
        <GnSearchFormField 
          v-model="searchTerm"
          borderColor="#d1d5db" 
          buttonBackgroundColor="#F97316" 
          color="white" 
          label="Search Sessions" 
          placeholder="Search Subjects"
          backgroundColor="rgb(28 25 23)"
          @gn-search="search"
        />
    </div>

    <br> </br>
    
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
