<template>
  <div class="container col-center">
    <BackBar text="Subject Name" />
    <br> </br>
    <div class="metrics-section">
      <div class="metrics-grid">
        <MetricCard metric="Daily Hours Reviewed" numeric=20 />
        <MetricCard metric="Questions Reviewed" numeric=20 />
        <MetricCard metric="Accuracy Rate" numeric=20 />
      </div>
    </div>
    <br />
    <SearchComponent text="Search Decks" :onClick="search" />
    <br></br>
    <DeckListView />
    <HoverButton icon="fa-solid fa-plus" :onClickHandler="createDeck" />
  </div>
</template>

<script>
import BackBar from "../../components/shared/BackBar.vue";
import SearchComponent from "../../components/shared/SearchComponent.vue";
import MetricCard from "../../components/shared/MetricCard.vue";
import HoverButton from "../../components/shared/HoverButton.vue";
import DeckListView from "../../components/deck/DeckListView.vue";

export default {
  name: "SubjectDetail",
  components: { BackBar, SearchComponent, MetricCard, HoverButton, DeckListView },
  methods: {
    createDeck() {
      this.$router.push({ path: "/deck/create" });
    },
    search() {
      console.log("search"); 
    }
  },
};
</script>

<style scoped>
.metrics-section {
  padding: 10px;
  margin: 10px;
  width: 90%;
  height: 100px;
}

.metrics-grid {
  display: flex;
  flex-direction: row;
  justify-content: center;
  align-items: center;
  width: 100%;
  height: 100%;
  gap: 1rem;
}
</style>
