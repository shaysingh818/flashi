<template>
  <div class="flex flex-col">

    <GnBackBar 
      	v-if="session" 
        :title="session.name"
        subtitle="Last updated 04/02/2026"
        textColor="var(--app-color-text)"
        color="var(--app-color-accent)"
        backgroundColor="var(--app-color-surface)"
        borderColor="var(--app-color-surface)"
        @gn-back="goBack">
        <font-awesome-icon icon="fa-solid fa-arrow-left" />  
    </GnBackBar>

    <div class="flex items-center justify-center">
      <QuestionTextPreview
        v-if="question"
        :text="question.raw"
      />
    </div>

    <div v-if="questionImageArtifacts.length === 1" class="flex justify-center gap-2 p-6">

      <div
	      v-for="img in questionImageArtifacts"
	      :key="img.id"
	      class="question-image question-image-single"
	      :style="{ backgroundImage: `url('${questionImageUrls[img.image_path]}')` }">

      </div>

    </div>

    <div v-if="questionImageArtifacts.length > 1" class="question-image-grid-container place-items-center">

      <div
	      v-for="img in questionImageArtifacts"
	      :key="img.id"
	      class="question-image question-image-grid"
	      :style="{ backgroundImage: `url('${questionImageUrls[img.image_path]}')` }">

      </div>

    </div>

    <div class="flex centered-all w-full">
      
      <OrderedSequencePreview 
        v-if="answerType === 'Ordered Sequence'" 
        :visible="false"
        :items="orderedSequences"
        @update:items="submittedOrderedSequences = $event"
      />

      <MultipleChoicePreview
        v-if="answerType === 'Multiple Choice'"
        :visible="false"
        :items="multipleChoiceAnswers"
        @select:item="selectedMultipleChoiceAnswers = $event"
      />
      
      <OpenEndedPreview
        v-if="answerType === 'Open Ended'"        
        :visible="false"
        :placeholderText="openEndedAnswer"
        :keywords="openEndedAnswerKeywords"
        @update:text="openEndedAnswer = $event"
      />
    </div>
  </div>

  <SessionActionsBar 
    :skipHandler="skipQuestion"
    :submitHandler="validateAnswer"
  />

  
</template>

<script setup>
import { readFile, BaseDirectory } from '@tauri-apps/plugin-fs';
import { onMounted, onBeforeUnmount, computed, reactive, ref, watch } from "vue";
import { useRoute, useRouter } from "vue-router";
import QuestionTextPreview from "../../components/question/QuestionTextPreview.vue";
import OrderedSequencePreview from "../../components/answer/ordered_sequence/OrderedSequencePreview.vue";
import MultipleChoicePreview from "../../components/answer/multiple_choice/MultipleChoicePreview.vue";
import OpenEndedPreview from "../../components/answer/open_ended/OpenEndedPreview.vue";
import SessionActionsBar from "../../components/session/SessionActionsBar.vue";
import {
  deckStore,
  questionStore,
  imageArtifactStore,
  sessionStore,
  sessionReviewStore,
  getCurrentTimestamp,
  answerTypeStore,
  openEndedStore,
  orderedSequenceStore,
  multipleChoiceAnswerStore
} from "../../api";

const route = useRoute();
const router = useRouter();
const sessionQuestionId = computed(() => route.params.id);
const sessionId = computed(() => route.params.session_id);

const sessionQuestion = ref(null); 
const session = ref(null);
const sessionReview = ref(null); 
const question = ref(null);
const questionImageArtifacts = ref([]);
const questionImageUrls = reactive({});
const deck = ref(null); 
const answerType = ref('');

/* answers */
const openEndedAnswer = ref(''); 
const multipleChoiceAnswer = ref('');
const orderedSequences = ref([]); 
const multipleChoiceAnswers = ref([]); 

const submittedOrderedSequences = ref([]);
const selectedMultipleChoice = ref(null); 
const selectedMultipleChoiceAnswers = ref([]);

/* edit toggles */ 
const editQuestionOpen = ref(false); 
const isDropdownOpen = ref(false);

async function skipQuestion() {

    const next = await sessionReviewStore.nextQuestion();
    openEndedAnswer.value = ''; 
    try {
      await router.push({ path: `/sessions/review/${sessionId.value}/question/${next.id}` });
    } catch (err) {
      console.error('Navigation failed:', err);
    }
}

function showDropdown() {
  isDropdownOpen.value = !isDropdownOpen.value;
  if (!isDropdownOpen.value) {
    loadData(); 
  }
}

async function loadThumbnail(imageFileName) {
  try {
    const fileBuffer = await readFile(`flashi/images/${imageFileName}`, {
      baseDir: BaseDirectory.Data,
    });
    return URL.createObjectURL(new Blob([fileBuffer]));
  } catch (err) {
    console.log(`Failed to load thumbnail for ${imageFileName}`, err);
  }
}

async function loadData() {


  session.value = await sessionStore.viewSession(`${sessionId.value}`);
  sessionQuestion.value = await sessionReviewStore.viewSessionReviewQuestion(`${sessionQuestionId.value}`);
  sessionReview.value = await sessionReviewStore.viewSessionReview(`${sessionQuestion.value.session_review_id}`);

  question.value = await questionStore.viewQuestion(`${sessionQuestion.value.question_id}`);

  await imageArtifactStore.loadImageArtifacts(`${sessionQuestion.value.question_id}`);
  questionImageArtifacts.value = imageArtifactStore.images;

  for (const url of Object.values(questionImageUrls)) URL.revokeObjectURL(url);
  for (const key of Object.keys(questionImageUrls)) delete questionImageUrls[key];
  for (const img of questionImageArtifacts.value) {
    const url = await loadThumbnail(img.image_path);
    if (url) questionImageUrls[img.image_path] = url;
  }


  const answerTypeText = await answerTypeStore.viewAnswerType(`${question.value.answer_type_id}`);
  answerType.value = answerTypeText.name;
  deck.value = await deckStore.viewDeck(`${question.value.deck_id}`); 
  
  if (answerType.value === 'Ordered Sequence') {
    const answer = await orderedSequenceStore.viewOrderedSequenceAnswer(
      `${question.value.answer_id}`
    );
    orderedSequences.value = answer.sequence; 
  }
  
  if (answerType.value === 'Multiple Choice') {
    selectedMultipleChoiceAnswers.value = []; 
    await multipleChoiceAnswerStore.loadMultipleChoiceAnswers(
      `${question.value.answer_id}`
    );
    multipleChoiceAnswers.value = multipleChoiceAnswerStore.multipleChoiceAnswers; 
  }
  
  if (answerType.value === 'Open Ended') {
    const answer = await openEndedStore.viewOpenEndedAnswer(
      `${question.value.answer_id}`
    );
    openEndedAnswer.value = answer.answer;
  }
  
}


async function validateAnswer() {

  if (answerType.value === 'Open Ended') { 
    const [result, msg] = await sessionReviewStore.completeAnswerMetadata(
      `${question.value.answer_id}`,
      sessionQuestion.value,
      openEndedAnswer.value
    );  
  }

  if (answerType.value === 'Ordered Sequence') {
    const [result, msg] = await sessionReviewStore.completeAnswerMetadata(
      `${question.value.answer_id}`,
      sessionQuestion.value,
      undefined,      
      undefined,
      submittedOrderedSequences.value
    );
  }

  if (answerType.value === 'Multiple Choice') {
    const [result, msg] = await sessionReviewStore.completeAnswerMetadata(
      `${question.value.answer_id}`,
      sessionQuestion.value,
      undefined,
      selectedMultipleChoiceAnswers.value,
      undefined
    );  
  }

  const nextId = await sessionReviewStore.nextQuestion(`${sessionId.value}`, sessionReview.value);
  try {
    const sessionComplete = await sessionReviewStore.checkSessionReviewComplete(`${sessionReview.value.id}`);
    console.log('Session complete: ', sessionComplete); 
    if (sessionComplete === true) {

      const completeResult = await sessionReviewStore.editSession(
        `${sessionReview.value.id}`,
        true,
        sessionReview.value.cycle,
        sessionReview.value.current_question,
        sessionReview.value.start_time,
        getCurrentTimestamp()
      ); 
      if (completeResult === "Success") {
        console.log("Successfully edited session with completes status: ", sessionReview.value.id); 
      }

      const sessionMetricsResult = await sessionReviewStore.generateSessionReviewMetrics(`${sessionReview.value.id}`);
      if (sessionMetricsResult === "Success") {
        console.log("Successfully generated session review metrics: ", sessionReview.value.id); 
      }
      
      await router.push({ path: `/sessions/detail/${sessionId.value}` });
    } else {      
      await router.push({ path: `/sessions/review/${sessionId.value}/question/${nextId}` });
    }
  } catch (err) {
    console.log("Error completing session: ", err); 
  }

}

function goBack() {
  router.go(-1);
}

onMounted(() => loadData(route.params.session_id, route.params.id));

onBeforeUnmount(() => {
  for (const url of Object.values(questionImageUrls)) URL.revokeObjectURL(url);
});

watch(() => route.params.id, (newId) => {

  if (newId) {
    console.log("new session question: ", newId); 
    loadData();
  } else {
    console.log(selectedMultipleChoiceAnswers.value); 
  }
});

</script>


<style scoped>

.question-image {
  width: 100%;
  aspect-ratio: 4 / 3;
  background-size: cover;
  background-position: center;
  background-repeat: no-repeat;
  border-radius: 10px;
}

.question-image-single {
  max-width: 500px;
  max-height: 500px;
}

.question-image-grid-container {
  display: grid;
  grid-template-columns: repeat(2, max-content);
  column-gap: 0.5rem;
  row-gap: 0.5rem;
  padding: 0.5rem;
  justify-content: center; 
}

.question-image-grid {
  width: 340px;
  height: 220px;
  aspect-ratio: auto;
}

</style>
