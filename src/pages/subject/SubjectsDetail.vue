<template>
  <div class="flex col-center gap-2 w-100">
    <BackBar text="Subject Name" />

    <div class="grid cols-3 w-90 gap-6 place-center p-6">
      <MetricCard metric="Daily hours reviewed" numeric="20" />
      <MetricCard metric="Questions reviewed" numeric="20" />
      <MetricCard metric="Accuracy rate" numeric="20" />
    </div>
    
    <SearchComponent text="Search Decks" :onClick="search" />
    <DeckListView />
    <HoverButton icon="fa-solid fa-plus" :onClickHandler="createDeck" />
  </div>
</template>

<script>
import BackBar from "../../components/shared/navigation/BackBar.vue";
import SearchComponent from "../../components/shared/forms/SearchComponent.vue";
import HoverButton from "../../components/shared/buttons/HoverButton.vue";
import DeckListView from "../../components/deck/DeckListView.vue";
import MetricCard from "../../components/shared/analytics/MetricCard.vue";

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
