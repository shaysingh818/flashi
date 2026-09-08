<template>
    <div class="p-6">
        <div class="flex flex-col" v-for="(item, index) in props.decks" :key="index">
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
    </div>
</template>

<script setup>
import DeckListItem from "./DeckListItem.vue";
import { deckStore, sessionStore, sessionReviewStore } from "../../api";
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
    await message('Added deck to session review', { title: 'Recalli', kind: 'info'})
  } else { 
    await message('Could not add deck to session review', { title: 'Recalli', kind: 'info'})
  }
}

async function removeDeckFromSession(deckId) { 
  const result = await sessionStore.removeDeckFromSession(props.sessionId, deckId);
  if (result === "Success") {
    await message('Removed deck from session review', { title: 'Recalli', kind: 'info'})
  } else {    
    await message('Could not remove deck from session review', { title: 'Recalli', kind: 'info'})
  }
  emit("update:items", deckId); 
}

</script>

