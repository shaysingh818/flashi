<template>
  <div class="flex col-center w-100">
    <br />
    <div
      class="flex col-center rounded-2 gap-1 p-2"
      style="background-color: rgb(28 25 23); width: 90%"
    >
      <div class="flex row-center gap-2">
        <PlayButton icon="fa-solid fa-play" />
        <p>Resume current session</p>
      </div>

      <div class="flex row-center gap-4 w-100" style="height: 10px;">
        <ProgressBar progress="50" />  
        <p>50/100</p>
      </div>
      
    </div>
    <br />
    <SearchComponent text="Search review sessions" :onClick="search" />
    <br />
    <SessionReviewTable />
    <HoverButton icon="fa-solid fa-play" :onClickHandler="createSession" />
  </div>
</template>

<script>
import SessionReviewTable from "../../components/session/SessionReviewTable.vue";
import HoverButton from "../../components/shared/buttons/HoverButton.vue";
import PlayButton from "../../components/shared/buttons/PlayButton.vue";
import SearchComponent from "../../components/shared/forms/SearchComponent.vue";
import ProgressBar from "../../components/shared/progress/ProgressBar.vue";

export default {
  name: "SessionReview",
  components: {
    SessionReviewTable,
    HoverButton,
    PlayButton,
    SearchComponent,
    ProgressBar,
  },
  methods: {
    questionDetail() {
      this.$router.push({ path: "/question/view" });
    },
  },
};
</script>

<style scoped>
p {
  color: white;
  font-size: 13px;
  font-weight: bold;
}

.progress-bar {
  height: 7px;
  border-radius: 20px;
  background-color: #3d3d3d;
  width: 75%;
}

.progress-bar-progress {
  height: 7px;
  border-radius: 20px;
  background-color: #f97316;
  width: 10%;
  justify-content: left;
}
</style>
