<template>
  <div class="flex flex-col">

    <GnBackBar 
        v-if="deck"
        :title="deck.name"
        subtitle="Last updated 04/02/2026"
        textColor="var(--app-color-text)"
        color="var(--app-color-accent)"
        backgroundColor="var(--app-color-surface)"
        borderColor="var(--app-color-surface)"
        @gn-back="goBack">
        <font-awesome-icon icon="fa-solid fa-arrow-left" />  
    </GnBackBar>
    
    
    <div class="p-4"> 
        <GnSearchFormField 
          v-model="searchTerm"
          inputBorderColor="rgb(28 25 23)" 
          buttonBorderColor="#F97316"
          buttonBackgroundColor="#F97316" 
          color="#F97316" 
          label="Search Questions" 
          placeholder="Search Questions"
          backgroundColor="rgb(28 25 23)"
          @gn-search="search"
        />
    </div>
    <QuestionTableView :deckId="deckId"/>
    <HoverButton icon="fa-solid fa-plus" @click="createQuestion" />
  </div>
</template>

<script setup>
import { ref } from "vue";
import { onMounted } from "vue";
import { deckStore } from "../../api/Deck";
import { useRouter, useRoute } from "vue-router";
import { GnSearchFormField } from '@graphyne/web-components';
import HoverButton from "../../components/shared/buttons/HoverButton.vue";
import QuestionTableView from "../../components/question/QuestionTableView.vue";


const router = useRouter();
const route = useRoute();
const deckId = route.params.id;
const deck = ref(null); 

function createQuestion() {
  router.push({path: `/question/create/${deckId}`})
}

function search() {
  console.log("search")
}

function goBack() {
  router.go(-1);
}

onMounted(async () => {
  deck.value = await deckStore.viewDeck(`${deckId}`);
  console.log(deck.value);  
})
</script>

