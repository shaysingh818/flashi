<template>
  <div class="flex col-center w-100">
    <br />
    <SearchComponent text="Search decks" :onClick="search" />
    <br/>
    <DeckListView actionType="remove"/>
    <HoverButton icon="fa-solid fa-plus" :onClickHandler="addDeckToSession" />
  </div>
</template>

<script>
import SearchComponent from '../../components/shared/forms/SearchComponent.vue';
import HoverButton from '../../components/shared/buttons/HoverButton.vue';
import DeckListView from '../../components/deck/DeckListView.vue';

export default {
  name: "SessionDecks",
  components: {SearchComponent, HoverButton, DeckListView},
  methods: {
    questionDetail() {
      this.$router.push({ path: "/question/view" });
    },
    search(){
      console.log("Test"); 
    },
    addDeckToSession() {
      this.$router.push({ path: "/sessions/deck/add" });
    }
  }
};
</script>


<style scoped>

</style>