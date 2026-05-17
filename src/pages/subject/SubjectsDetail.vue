<template>
  <div class="flex col-center gap-2 w-100">
    <BackBar v-if="subject" :text="subject.name" />

    <div class="grid cols-3 w-90 gap-6 place-center p-6">
      <MetricCard metric="Daily hours reviewed" numeric="20" />
      <MetricCard metric="Questions reviewed" numeric="20" />
      <MetricCard metric="Accuracy rate" numeric="20" />
    </div>

    <SearchComponent text="Search Decks" @search="search" />
    <DeckListView v-if="subject" :subjectId="`${subject.id}`" />
    <HoverButton icon="fa-solid fa-plus" :onClickHandler="createDeck" />
  </div>
</template>

<script setup>
import { ref } from "vue";
import { useRoute, useRouter } from "vue-router";
import { onMounted } from "vue";
import { subjectStore } from "../../api/Subject";
import { deckStore } from "../../api/Deck";
import BackBar from "../../components/shared/navigation/BackBar.vue";
import SearchComponent from "../../components/shared/forms/SearchComponent.vue";
import HoverButton from "../../components/shared/buttons/HoverButton.vue";
import DeckListView from "../../components/deck/DeckListView.vue";
import MetricCard from "../../components/shared/analytics/MetricCard.vue";
import { definition } from "@fortawesome/free-solid-svg-icons/faRocket";

const route = useRoute();
const router = useRouter();
const searchTerm = ref("");
const subjectId = route.params.id;
const subject = ref(null);

onMounted(async () => {
  subject.value = await subjectStore.viewSubject(subjectId);
});

function createDeck() {
  router.push({ path: `/deck/create/${subjectId}` });
}

async function search(searchTerm) {
  await deckStore.searchDecks(searchTerm);
}
</script>
