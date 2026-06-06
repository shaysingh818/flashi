<template>
  <div class="flex col-center w-100">
    <BackBar 
      v-if="deck" 
      :text="deck.name"
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
        :visible="true"
        :items="orderedSequences"
        @update:items="updatedOrderedSequences = $event;"
        @update:delete="updatedOrderedSequences = $event; editAnswer()"
        @update:add="updatedOrderedSequences = $event; editAnswer()"
      />

      <MultipleChoicePreview 
        v-if="answerType === 'Multiple Choice'"
        :visible="true"
        :items="multipleChoiceAnswers"
        @update:item="updatedMultipleChoiceAnswer = $event; appendChoice()"
        @update:add="multipleChoiceAnswer = $event; editAnswer()"
        @update:delete="multipleChoiceDeleteIndex = $event; editAnswer()"
      />
      
      <OpenEndedPreview
        v-if="answerType === 'Open Ended'"        
        :visible="true"
        :placeholderText="openEndedAnswer"
        :keywords="openEndedAnswerKeywords"
        @update:keywords="openEndedAnswerKeywords = $event"
        @update:text="openEndedAnswer = $event"
      />
    </div>
  </div>

  <QuestionActionsBar 
    :previousHandler="previousQuestion"
    :updateHandler="editAnswer"
    :nextHandler="nextQuestion"
  />

  <EditQuestionForm 
    v-if="isDropdownOpen"
    :questionId="`${ question.id }`"
    :questionText="question.raw"
    :closeFormHandler="showDropdown"
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
import QuestionActionsBar from "./QuestionActionsBar.vue";
import EditQuestionForm from "../../components/question/EditQuestionForm.vue";
import { 
  deckStore,
  questionStore,
  answerTypeStore,
  openEndedStore,
  orderedSequenceStore,
  multipleChoiceStore,
  multipleChoiceAnswerStore
} from "../../api";

const route = useRoute();
const router = useRouter();
const questionId = route.params.id;
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


async function nextQuestion() {

  /*
  const result = await multipleChoiceStore.validateMultipleChoice(
    `${question.value.answer_id}`, 
    ["20"]
  );
  console.log(result); */

  questionStore.cycleNextQuestion();
  const currId = questionStore.questions[questionStore.getCurrQuestion()].id;
  router.push({path: `/question/view/${currId}`, replace: true});
}

function previousQuestion() {
  questionStore.cyclePreviousQuestion();
  const currId = questionStore.questions[questionStore.getCurrQuestion()].id;
  router.push({path: `/question/view/${currId}`, replace: true});
}

function appendChoice() {

  if(updatedMultipleChoiceAnswers.value.length > 0) {
    let idExists = false;
    const currId = updatedMultipleChoiceAnswer.value.id;
    for(var i = 0; i < updatedMultipleChoiceAnswers.value.length; i++) {
      const currItem = updatedMultipleChoiceAnswers.value[i]; 
      if(currItem.id === currId) {
        updatedMultipleChoiceAnswers.value[i] = updatedMultipleChoiceAnswer.value;
        updatedMultipleChoiceAnswer.value = null;
        idExists = true;
      }
    }
    if(!idExists) {
      updatedMultipleChoiceAnswers.value.push(updatedMultipleChoiceAnswer.value); 
      updatedMultipleChoiceAnswer.value = null;
    }
  } else {
    updatedMultipleChoiceAnswers.value.push(updatedMultipleChoiceAnswer.value); 
    updatedMultipleChoiceAnswer.value = null;
  }

}

function showDropdown() {
  isDropdownOpen.value = !isDropdownOpen.value;
  if (!isDropdownOpen.value) {
    loadData(); 
  }
}

async function loadData(id) {

  question.value = await questionStore.viewQuestion(`${id}`);
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


async function editAnswer() {

  if (answerType.value === 'Open Ended') {

    const editResult = await openEndedStore.editOpenEndedAnswer(
        `${question.value.answer_id}`,
        openEndedAnswer.value, 
        '',
        openEndedAnswerKeywords.value 
    );
    loadData(question.value.id);
  }

  if (answerType.value === 'Ordered Sequence') {

    const preservedAnswerOrder = [];
    for (const [index, item] of updatedOrderedSequences.value.entries()) {
      preservedAnswerOrder.push(item);
    }

    const editResult = await orderedSequenceStore.editOrderedSequenceAnswer(
      `${question.value.answer_id}`,
      preservedAnswerOrder,
      '',
    );
    loadData(question.value.id);
  }

  if (answerType.value === 'Multiple Choice') {

    if(multipleChoiceAnswer.value != '') {
      await multipleChoiceAnswerStore.createMultipleChoiceAnswer( 
        `${question.value.answer_id}`,
        multipleChoiceAnswer.value,
        '',
        false
      );
      loadData(question.value.id);
      multipleChoiceAnswer.value = '';
    }

    if(multipleChoiceDeleteIndex.value != null) {
      const multipleChoiceAnswer = multipleChoiceAnswers.value[multipleChoiceDeleteIndex.value];
      await multipleChoiceAnswerStore.deleteMultipleChoiceAnswer(
        `${multipleChoiceAnswer.id}`
      );
      loadData(question.value.id);
      multipleChoiceDeleteIndex.value = null;
    }



    if (updatedMultipleChoiceAnswers.value.length > 0) {
      for(const item of updatedMultipleChoiceAnswers.value) {  
        await multipleChoiceAnswerStore.editMultipleChoiceAnswer(
          `${item.id}`,
          `${item.answer_id}`,
          item.answer, 
          '',
          item.is_correct
        );
      }
      loadData(question.value.id);
      updatedMultipleChoiceAnswers.value = [];
    }

  }

}

onMounted(() => loadData(route.params.id)); 

watch(() => route.params.id, (newId) => {
  if (newId) loadData(newId);
});

</script>
