<template>
  <div class="flex col-center w-100">
    <BackBar v-if="deck" :text="deck.name" />
    <br></br>
    
    <div class="w-85"> 
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
    <br> </br>
    <QuestionTableView :deckId="deckId"/>
    <HoverButton icon="fa-solid fa-plus" @click="createQuestion" />
  </div>
</template>

<script setup>
import { ref } from "vue";
import { onMounted } from "vue";
import { deckStore } from "../../api/Deck";
import { useRouter, useRoute } from "vue-router";
import BackBar from "../../components/shared/navigation/BackBar.vue";
import { GnSearchFormField } from '@graphyne/web-components';
import MetricCard from "../../components/shared/MetricCard.vue";
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

onMounted(async () => {
  deck.value = await deckStore.viewDeck(`${deckId}`);
  console.log(deck.value);  
})
</script>

