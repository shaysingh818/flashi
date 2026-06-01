<template>
  <div class="flex col-center w-100">
    <br />
    <SearchComponent text="Search decks" :onClick="search" />
    <br/>

    <DeckListView 
      :decks="decks" 
      :sessionId="sessionId"
      actionType="remove"
      @update:items="handleUpdate()"
    />

    
    <HoverButton icon="fa-solid fa-plus" @click="addDeckToSession" />
  </div>
</template>

<script setup>
import { onMounted, ref } from "vue"; 
import { useRouter, useRoute } from "vue-router";
import { sessionStore } from '../../api';
import SearchComponent from '../../components/shared/forms/SearchComponent.vue';
import HoverButton from '../../components/shared/buttons/HoverButton.vue';
import DeckListView from '../../components/deck/DeckListView.vue';

const router = useRouter();
const route = useRoute(); 
const sessionId = route.params.id;
const decks = ref([]);


async function loadData() {
  decks.value = await sessionStore.viewSessionDecks(sessionId);
}

async function handleUpdate() {
  await loadData();
}

async function addDeckToSession() {
  router.push({ path: `/sessions/deck/add/${sessionId}` });
}

async function search() {

}


onMounted(loadData); 
</script>

