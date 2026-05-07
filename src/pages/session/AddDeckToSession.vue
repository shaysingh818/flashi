<template>
  <div class="flex col-center w-100">
    <BackBar text="Add Deck to session name" />
    <br />
    <SearchComponent text="Search decks to add" :onClick="search" />
    <br/>
    <DeckListView actionType="add"/>
  </div>
</template>

<script>
import SearchComponent from '../../components/shared/forms/SearchComponent.vue';
import DeckListView from '../../components/deck/DeckListView.vue';
import BackBar from '../../components/shared/navigation/BackBar.vue';

export default {
  name: "AddDeckToSession",
  components: {SearchComponent, DeckListView, BackBar},
  methods: {
    questionDetail() {
      this.$router.push({ path: "/question/view" });
    },
    search(){
      console.log("Test"); 
    }
  }
};
</script>

