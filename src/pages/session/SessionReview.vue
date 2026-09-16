<template>
  <div class="flex flex-col items-center p-6">
    <div
      class="flex flex-col gap-1 p-4 items-center justify-center w-full"
      style="background-color: var(--app-color-surface); border-radius: 5px;"
    >
      <div class="flex flex-row gap-2 items-center justify-center">

        <GnIconButton label="Add tag" variant="primary" size="sm" @gn-click="startSessionReview">
            <font-awesome-icon icon="fa-solid fa-play" />
        </GnIconButton>
        <p>{{ sessionProgressMessage }}</p>
      </div>

      <div class="flex items-center justify-center gap-4 w-full" style="height: 10px;">
        <ProgressBar :progress="completionPercentage" />
        <p>{{ completed }}/{{ total }}</p>
      </div>

    </div>
    <br />
    <SessionReviewTable :sessionId="props.sessionId" />
    <HoverButton icon="fa-solid fa-play" @click="startSessionReview" />
  </div>
</template>

<script setup>
import { ref, onMounted } from "vue"; 
import { useRouter } from "vue-router";
import { message } from '@tauri-apps/plugin-dialog'; 
import { sessionReviewStore, sessionStore } from "../../api";
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
const completed = ref(0); 
const total = ref(0); 
const completionPercentage = ref(0.00); 
const sessionProgressMessage = ref(''); 

async function questionDetail() {
  router.push({ path: "/question/view" });
}

async function startSessionReview() {
  
  const existingSession = await sessionReviewStore.checkSessionReviewActive(`${props.sessionId}`);
  sessionReviewStore.current_question = 0;
  const sessionDecks = await sessionStore.viewSessionDecks(`${props.sessionId}`);
 
  if (sessionDecks.length > 0) { 
    if (existingSession === -1) {
      const session = await sessionReviewStore.createSessionReview(`${props.sessionId}`);
      await sessionReviewStore.loadActiveSessionReviewQuestions(`${props.sessionId}`);
      const firstQuestion = sessionReviewStore.session_review_questions[0];
      router.push({ path: `/sessions/review/${props.sessionId}/question/${firstQuestion.id}` });
    } else {
      await sessionReviewStore.loadActiveSessionReviewQuestions(`${props.sessionId}`);
      console.log(sessionReviewStore.session_review_questions);
      const firstQuestion = sessionReviewStore.session_review_questions[0];
      router.push({ path: `/sessions/review/${props.sessionId}/question/${firstQuestion.id}` });
    }
 
  } else {
    await message(
      'There are no decks added to this session. Add decks to the session in order to start a review session',
      { title: 'Recalli', kind: 'info' }
    );
  }
  
}

onMounted(async () => {

  const existingSession = await sessionReviewStore.checkSessionReviewActive(`${props.sessionId}`);
  if (existingSession === -1) {
    sessionProgressMessage.value = "Start a new session review"; 
  } else {
    sessionProgressMessage.value = "Resume current session"; 
  }
  
  const [sComplete, sTotal] = await sessionStore.viewSessionProgressStatus(`${props.sessionId}`); 
  completed.value = sComplete; 
  total.value = sTotal;
  completionPercentage.value = completed.value / total.value * 100; 
}); 

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
