<template>
  <div class="flex col-center w-100">
    <br />
    <div
      class="flex col-center rounded-2 gap-1 p-4"
      style="background-color: rgb(28 25 23); width: 90%"
    >
      <div class="flex row-center gap-2">
        <PlayButton icon="fa-solid fa-play" @click="startSessionReview" />
        <p>Resume current session</p>
      </div>

      <div class="flex row-center gap-4 w-100" style="height: 10px;">
        <ProgressBar progress="50" />  
        <p>50/100</p>
      </div>
      
    </div>
    <br />
    <SessionReviewTable />
    <HoverButton icon="fa-solid fa-play" @click="createSession" />
  </div>
</template>

<script setup>
import { useRouter } from "vue-router";
import { sessionReviewStore } from "../../api"; 
import SessionReviewTable from "../../components/session/SessionReviewTable.vue";
import HoverButton from "../../components/shared/buttons/HoverButton.vue";
import PlayButton from "../../components/shared/buttons/PlayButton.vue";
import SearchComponent from "../../components/shared/forms/SearchComponent.vue";
import ProgressBar from "../../components/shared/progress/ProgressBar.vue";


const props = defineProps({
  sessionId: {
    type: String,
    required: true,
  },
});

const router = useRouter();

async function questionDetail() {
  router.push({ path: "/question/view" });
}

async function startSessionReview() {
  console.log("starting session review"); 
  const session = await sessionReviewStore.createSessionReview(`${props.sessionId}`);
  console.log(session);
  await sessionReviewStore.loadSessionReviewQuestions(`${session.id}`);
  console.log(sessionReviewStore.session_review_questions);
}

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
