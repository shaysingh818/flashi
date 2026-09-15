<template>
  <div class="flex flex-col">
    <GnBackBar 
        title="Add deck to session"
        subtitle="Last updated 04/02/2026"
        textColor="var(--app-color-text)"
        color="var(--app-color-accent)"
        backgroundColor="var(--app-color-surface)"
        borderColor="var(--app-color-surface)"
        @gn-back="goBack">
        <font-awesome-icon icon="fa-solid fa-arrow-left" />  
    </GnBackBar>

    <div class="p-6">
      <GnSearchFormField 
        v-model="searchTerm"
        inputBorderColor="rgb(28 25 23)" 
        buttonBorderColor="var(--app-color-accent)"
        buttonBackgroundColor="var(--app-color-accent)" 
        buttonIconColor="var(--app-color-text)"
        focusColor="var(--app-color-accent)" 
        textColor="var(--app-color-accent)"
        label="Search decks to add" 
        placeholder="Search decks to add"
        backgroundColor="var(--app-color-surface)"
        @gn-search="search"
      />
    </div>

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


const route = useRoute(); 
const sessionId = route.params.id;

onMounted(async () => {
  await deckStore.loadDecks();
});

async function search(searchTerm) {
  await deckStore.searchDecks(searchTerm);
}

function goBack() {
  router.go(-1);
}

</script>

