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
        @update:items="updatedOrderedSequences = $event;"
        @update:delete="updatedOrderedSequences = $event; editAnswer()"
        @update:add="updatedOrderedSequences = $event; editAnswer()"
      />

      <MultipleChoicePreview 
        v-if="answerType === 'Multiple Choice'"
        :visible="false"
        :items="multipleChoiceAnswers"
        @update:item="updatedMultipleChoiceAnswer = $event; appendChoice()"
        @update:add="multipleChoiceAnswer = $event; editAnswer()"
        @update:delete="multipleChoiceDeleteIndex = $event; editAnswer()"
      />
      
      <OpenEndedPreview
        v-if="answerType === 'Open Ended'"        
        :visible="false"
        :placeholderText="openEndedAnswer"
        :keywords="openEndedAnswerKeywords"
        @update:keywords="openEndedAnswerKeywords = $event"
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
import { onMounted, ref, toRaw, watch } from "vue";
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
const sessionQuestionId = route.params.id;
const sessionId = route.params.session_id;

const sessionQuestion = ref(null); 
const session = ref(null);
const question = ref(null);
const deck = ref(null); 
const answerType = ref('');

/* answers */
const openEndedAnswer = ref(''); 
const multipleChoiceAnswer = ref('');
const multipleChoiceDeleteIndex = ref(null);
const openEndedAnswerKeywords = ref([]);
const orderedSequences = ref([]); 
const multipleChoiceAnswers = ref([]); 
const newMultipleChoiceAnswers = ref([]);

/* updated answers */
const updatedOrderedSequences = ref([]);
const updatedMultipleChoiceAnswer = ref(null); 
const updatedMultipleChoiceAnswers = ref([]); 

/* edit toggles */ 
const editQuestionOpen = ref(false); 
const isDropdownOpen = ref(false);


async function skipQuestion() {

}

function showDropdown() {
  isDropdownOpen.value = !isDropdownOpen.value;
  if (!isDropdownOpen.value) {
    loadData(); 
  }
}

async function loadData(id) {

  session.value = await sessionStore.viewSession(`${sessionId}`);
  sessionQuestion.value = await sessionReviewStore.viewSessionReviewQuestion(`${sessionQuestionId}`);
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
    openEndedAnswerKeywords.value = answer.keywords;
  }
}


async function validateAnswer() {

  if (answerType.value === 'Open Ended') {

  }

  if (answerType.value === 'Ordered Sequence') {

  }

  if (answerType.value === 'Multiple Choice') {

  }

}

onMounted(() => loadData(route.params.id)); 

watch(() => route.params.id, (newId) => {
  if (newId) loadData(newId);
});

</script>
