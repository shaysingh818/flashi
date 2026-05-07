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

<script>
import BackBar from "../../components/shared/navigation/BackBar.vue";
import SearchComponent from "../../components/shared/forms/SearchComponent.vue";
import MetricCard from "../../components/shared/MetricCard.vue";
import HoverButton from "../../components/shared/buttons/HoverButton.vue";
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
