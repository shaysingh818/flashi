<template>
  <div class="flex flex-col">

    <GnBackBar 
        v-if="subject"
        :title="subject.name"
        subtitle="Last updated 04/02/2026"
        textColor="var(--app-color-text)"
        color="var(--app-color-accent)"
        backgroundColor="var(--app-color-surface)"
        borderColor="var(--app-color-surface)"
        @gn-back="goBack">
        <font-awesome-icon icon="fa-solid fa-arrow-left" />  
    </GnBackBar>

    <div class="grid grid-cols-3 gap-4 p-4">
      <GnMetricCard metric="Average Accuracy" :numeric="avgAccuracy" />
      <GnMetricCard metric="Questions reviewed" :numeric="questionsReviewed" />
      <GnMetricCard metric="Total Questions" :numeric="questionsTotal" />
    </div>
    
    <div class="p-4"> 
        <GnSearchFormField 
          v-model="searchTerm"          
          label="Search Decks" 
          placeholder="Search Decks"
          inputBorderColor="rgb(28 25 23)" 
          buttonBorderColor="var(--app-color-accent)"
          buttonBackgroundColor="#F97316"          
          buttonIconColor="var(--app-color-text)"          
          focusColor="var(--app-color-accent)" 
          textColor="var(--app-color-accent)"
          backgroundColor="var(--app-color-surface)"
          @gn-search="search"
        />
    </div>
    <DeckListView 
      v-if="subject" 
      :decks="deckStore.decks" 
      actionType="view"
    />
    <HoverButton icon="fa-solid fa-plus" @click="createDeck" />
  </div>
</template>

<script setup>
import { ref } from "vue";
import { useRoute, useRouter } from "vue-router";
import { onMounted } from "vue";
import { subjectStore } from "../../api/Subject";
import { deckStore } from "../../api/Deck";
import BackBar from "../../components/shared/navigation/BackBar.vue";
import { GnSearchFormField, GnMetricCard, GnBackBar } from '@graphyne/web-components';
import HoverButton from "../../components/shared/buttons/HoverButton.vue";
import DeckListView from "../../components/deck/DeckListView.vue";
import MetricCard from "../../components/shared/analytics/MetricCard.vue";
import { definition } from "@fortawesome/free-solid-svg-icons/faRocket";

const route = useRoute();
const router = useRouter();
const searchTerm = ref("");
const subjectId = route.params.id;
const subject = ref(null);
const avgAccuracy = ref(0.0);
const questionsReviewed = ref(0); 
const questionsTotal = ref(0); 

onMounted(async () => {
  subject.value = await subjectStore.viewSubject(subjectId);
  await deckStore.loadDecksBySubject(subjectId);

  [
    questionsReviewed.value,
    questionsTotal.value,    
    avgAccuracy.value,
  ] = await subjectStore.subjectMetrics(subjectId);
  
  avgAccuracy.value = Math.round(avgAccuracy.value * 100) / 100;
  
});

function goBack() {
  router.go(-1);
}

function createDeck() {
  router.push({ path: `/deck/create/${subjectId}` });
}

async function search(searchTerm) {
  await deckStore.searchDecks(searchTerm);
}
</script>
