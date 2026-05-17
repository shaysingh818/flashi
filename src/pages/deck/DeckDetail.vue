<template>
  <div class="flex col-center">
    <BackBar text="Deck Name" />
    <br></br>
    <SearchComponent text="Search Decks" :onClick="search" />
    <br> </br>
    <QuestionTableView />
    <HoverButton icon="fa-solid fa-plus" :onClickHandler="createQuestion" />
  </div>
</template>

<script setup>
import { ref } from "vue";
import { useRouter, useRoute } from "vue-router";
import BackBar from "../../components/shared/navigation/BackBar.vue";
import SearchComponent from "../../components/shared/forms/SearchComponent.vue";
import MetricCard from "../../components/shared/MetricCard.vue";
import HoverButton from "../../components/shared/buttons/HoverButton.vue";
import QuestionTableView from "../../components/question/QuestionTableView.vue";


const router = useRouter();
const route = useRoute();
const deckId = route.params.id;

function createQuestion() {
  router.push({path: `/question/create/${deckId}`})
}

function search() {
  console.log("search")
}

</script>

