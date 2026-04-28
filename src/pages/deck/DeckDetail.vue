<template>

  <div class="container col-center">
    <BackBar text="Deck Name" />
    <br></br>
    <SearchComponent text="Search Decks" :onClick="search" />

    <br> </br>

    <QuestionTableView />

    <HoverButton icon="fa-solid fa-plus" :onClickHandler="createQuestion" />


  </div>

</template>

<script>
import BackBar from "../../components/shared/BackBar.vue";
import SearchComponent from "../../components/shared/SearchComponent.vue";
import MetricCard from "../../components/shared/MetricCard.vue";
import HoverButton from "../../components/shared/HoverButton.vue";
import QuestionTableView from "../../components/question/QuestionTableView.vue";

export default {
  name: "DeckDetail",
  components: { BackBar, SearchComponent, MetricCard, HoverButton, QuestionTableView },
  methods: {
    createQuestion() {
      this.$router.push({ path: "/question/create" });
    },
    search() {
      console.log("search");
    },
  },
};
</script>

<style scoped></style>
