<template>
  <div class="deck-list-view" v-for="(item, index) in props.decks" :key="index">
    <DeckListItem
      :id="`${item.id}`"
      :subjectId="`${item.subject_id}`"
      :title="item.name"
      :description="item.description"      
      :tags="item.tags"
      :dateUpdated="item.updated"
      :actionType="props.actionType"
      @deck:add_to_session="addDeckToSession($event)"
      @deck:remove_from_session="removeDeckFromSession($event)"
    />
  </div>
</template>

<script setup>
import DeckListItem from "./DeckListItem.vue";
import { deckStore, sessionStore } from "../../api";
import { message } from '@tauri-apps/plugin-dialog'; 


const props = defineProps({
  decks: {
    type: Array,
    default: () => [],
  },
  actionType: {
    type: String,
    default: () => "view",
  }, 
  sessionId: {
    type: String,
    required: false,
  },
});


const emit = defineEmits(["update:items"]);

async function addDeckToSession(deckId) {
  const result = await sessionStore.addDeckToSession(props.sessionId, deckId);
  if (result === "Success") {
    await message('Added deck to session review', { title: 'Alert', kind: 'info'})
  } else { 
    await message('Could not add deck to session review', { title: 'Alert', kind: 'info'})
  }
}

async function removeDeckFromSession(deckId) {
  const result = await sessionStore.removeDeckFromSession(props.sessionId, deckId);
  if (result === "Success") {
    alert("Removed deck from session review");
  } else {
    alert("Could not remove deck from session review"); 
  }
  emit("update:items", deckId); 
}

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
