<template>
  <div class="deck-list-view" v-for="(item, index) in deckStore.decks" :key="index">
    <DeckListItem
      :id="`${item.id}`"
      :subjectId="`${item.subject_id}`"
      :title="item.name"
      :description="item.description"      
      :tags="item.tags"
      :dateUpdated="item.updated"
      actionType="add"
      @deck:add_to_session="addDeckToSession($event)"
    />
  </div>
</template>

<script setup>
import { useRoute } from "vue-router";
import DeckListItem from "../deck/DeckListItem.vue";
import { deckStore, sessionStore } from "../../api";
import { onMounted } from "vue";

const props = defineProps({
    id: {
      type: String,
      required: true,
    },
});

async function addDeckToSession(deckId) {
  await sessionStore.addDeckToSession(props.id, deckId);
}

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
