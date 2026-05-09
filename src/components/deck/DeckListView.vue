<template>
  <div class="deck-list-view" v-for="(item, index) in deckStore.decks" :key="index">
    <DeckListItem
      :id="item.id"
      :title="item.name"
      :description="item.description"      
      :tags="item.tags"
      :dateUpdated="item.updated"
      :actionType="actionType"
    />

  </div>
</template>

<script setup>
import DeckListItem from "./DeckListItem.vue";
import { deckStore } from "../../api/Deck";
import { onMounted } from "vue";

const props = defineProps({
  actionType: {
    type: String,
    default: () => "view",
  },
});

onMounted(async () => {
  await deckStore.loadDecks();
});

</script>

<style>
.deck-list-view {
  display: flex;
  flex-direction: column;
  gap: 1rem;
  width: 85%;
  padding: 10px;
  justify-content: center;
  align-items: center;
}
</style>
