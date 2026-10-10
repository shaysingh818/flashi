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
          label="Search Decks" 
          placeholder="Search Decks In Session"
          backgroundColor="var(--app-color-surface)"
          @gn-search="search"
        />
    </div>

    <DeckListView 
      :decks="decks" 
      :sessionId="props.sessionId"
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
import HoverButton from '../../components/shared/buttons/HoverButton.vue';
import DeckListView from '../../components/deck/DeckListView.vue';

const router = useRouter();
const route = useRoute(); 
const decks = ref([]);

const props = defineProps({
  sessionId: {
    type: String,
    required: true,
  },
});


async function loadData() {
  decks.value = await sessionStore.viewSessionDecks(props.sessionId);
}

async function handleUpdate() {
  await loadData();
}

async function addDeckToSession() {
  router.push({ path: `/sessions/subject/add/${props.sessionId}` });
}

async function search() {

}

onMounted(loadData); 
</script>

