<template>
  <div class="flex col-center w-100">
    <BackBar 
      v-if="session" 
      :text="session.name"
      :actions="[
         { icon: 'fa-solid fa-pen-to-square', handler: showDropdown },
      ]"
    />

    <div class="flex centered-all w-100">
      <QuestionTextPreview
        v-if="question"
        :text="question.raw"
      />
    </div>
    

    <div class="flex centered-all w-100">
      
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
import { onMounted, computed, ref, toRaw, watch } from "vue";
import { useRoute, useRouter } from "vue-router";
import BackBar from "../../components/shared/navigation/BackBar.vue";
import QuestionTextPreview from "../../components/question/QuestionTextPreview.vue";
import OrderedSequencePreview from "../../components/answer/ordered_sequence/OrderedSequencePreview.vue";
import MultipleChoicePreview from "../../components/answer/multiple_choice/MultipleChoicePreview.vue";
import OpenEndedPreview from "../../components/answer/open_ended/OpenEndedPreview.vue";
import SessionActionsBar from "../../components/session/SessionActionsBar.vue";
import EditQuestionForm from "../../components/question/EditQuestionForm.vue";
import { 
  deckStore,
  questionStore,
  sessionStore,
  sessionReviewStore,
  answerTypeStore,
  openEndedStore,
  orderedSequenceStore,
  multipleChoiceStore,
  multipleChoiceAnswerStore
} from "../../api";

const route = useRoute();
const router = useRouter();
const sessionQuestionId = computed(() => route.params.id);
const sessionId = computed(() => route.params.session_id);

const sessionQuestion = ref(null); 
const session = ref(null);
const question = ref(null);
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

async function loadData() {

  session.value = await sessionStore.viewSession(`${sessionId.value}`);
  sessionQuestion.value = await sessionReviewStore.viewSessionReviewQuestion(`${sessionQuestionId.value}`);
  question.value = await questionStore.viewQuestion(`${sessionQuestion.value.question_id}`);
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

    const [result, msg] = await openEndedStore.validateOpenEndedAnswer(
      `${question.value.answer_id}`, 
      openEndedAnswer.value
    );

    
    if(result === true) {

      

      /* update database with stats associated with question */
      openEndedAnswer.value = '';
      const next = await sessionReviewStore.nextQuestion();
      try {
        await router.push({ path: `/sessions/review/${sessionId.value}/question/${next.id}` });
      } catch (err) {
        console.error('Navigation failed:', err);
      }

    }

  }

  if (answerType.value === 'Ordered Sequence') {

    const sequenceAnswers = submittedOrderedSequences.value.map(item => item.item.toString());
    const [result, msg] = await orderedSequenceStore.validateOrderedSequenceAnswer(
      `${question.value.answer_id}`, 
      sequenceAnswers
    );

    if(result === true) {
      submittedOrderedSequences.value = [];
      const next = await sessionReviewStore.nextQuestion();
      try {
        await router.push({ path: `/sessions/review/${sessionId.value}/question/${next.id}` });
      } catch (err) {
        console.error('Navigation failed:', err);
      }

    } else {
      console.log("Incorrect"); 
    }

  }

  if (answerType.value === 'Multiple Choice') {

    const answerIds = selectedMultipleChoiceAnswers.value.map(item => item.id.toString());
    const [result, msg] = await multipleChoiceStore.validateMultipleChoice(
      `${question.value.answer_id}`, 
      answerIds
    );


    if(result === true) {

      /* notify user they were correct */

      /* update database with stats associated with question */

      const next = await sessionReviewStore.nextQuestion();
      try {
        await router.push({ path: `/sessions/review/${sessionId.value}/question/${next.id}` });
      } catch (err) {
        console.error('Navigation failed:', err);
      }

    } else {

    }

  }

}

onMounted(() => loadData(route.params.session_id, route.params.id)); 

watch(() => route.params.id, (newId) => {

  if (newId) {
    console.log("new session question: ", newId); 
    loadData();
  } else {
    console.log(selectedMultipleChoiceAnswers.value); 
  }
});

</script>
