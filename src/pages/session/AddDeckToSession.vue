<template>
  <div class="flex col-center w-100">
    <BackBar text="Add deck to session" />
    <br />
    <SearchComponent text="Search decks to add" @search="search" />
    <br/>
    <DeckListView 
      :decks="deckStore.decks"
      :sessionId="sessionId"
      actionType="add"
    />
  </div>
</template>

<script setup>
import { ref, onMounted } from "vue"; 
import { useRoute } from "vue-router";
import { deckStore } from "../../api"; 
import SearchComponent from '../../components/shared/forms/SearchComponent.vue';
import DeckListView from '../../components/deck/DeckListView.vue';
import BackBar from '../../components/shared/navigation/BackBar.vue';
import SessionDecksToAdd from '../../components/session/SessionDecksToAdd.vue';


const route = useRoute(); 
const sessionId = route.params.id;

onMounted(async () => {
  await deckStore.loadDecks();
});

async function search(searchTerm) {
  await deckStore.searchDecks(searchTerm);
}

</script>

