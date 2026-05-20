<template>
  <div class="flex col-center w-100">
    <BackBar v-if="deck" :text="deck.name" />

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
      />
      <MultipleChoicePreview 
        v-if="answerType === 'Multiple Choice'"
        :visible="true"
        :items="multipleChoiceAnswers"
      />
      
      <OpenEndedPreview
        v-if="answerType === 'Open Ended'"        
        :visible="true"
        :placeholderText="openEndedAnswer"
        :keywords="openEndedAnswerKeywords"
      />
    </div>
  </div>

  <QuestionActionsBar />
</template>

<script setup>
import { onMounted, ref } from "vue";
import { useRoute, useRouter } from "vue-router";
import { questionStore } from "../../api/Question";
import { answerTypeStore } from "../../api/AnswerType";
import { deckStore } from "../../api/Deck";
import { orderedSequenceStore } from "../../api/OrderedSequenceAnswer";
import BackBar from "../../components/shared/navigation/BackBar.vue";
import QuestionTextPreview from "../../components/question/QuestionTextPreview.vue";
import OrderedSequencePreview from "../../components/answer/ordered_sequence/OrderedSequencePreview.vue";
import MultipleChoicePreview from "../../components/answer/multiple_choice/MultipleChoicePreview.vue";
import OpenEndedPreview from "../../components/answer/open_ended/OpenEndedPreview.vue";
import QuestionActionsBar from "./QuestionActionsBar.vue";
import { multipleChoiceStore, multipleChoiceAnswerStore } from "../../api/MutlipleChoiceAnswer";
import { openEndedStore } from "../../api/OpenEndedAnswer";

const route = useRoute(); 
const questionId = route.params.id;
const question = ref(null); 
const deck = ref(null); 
const answerType = ref(''); 
const orderedSequences = ref([]); 
const multipleChoiceAnswers = ref([]); 
const openEndedAnswer = ref(''); 
const openEndedAnswerKeywords = ref([]);

onMounted(async () => {
  
  question.value = await questionStore.viewQuestion(`${questionId}`);
  const answerTypeText = await answerTypeStore.viewAnswerType(`${question.value.answer_type_id}`);
  answerType.value = answerTypeText.name;

  deck.value = await deckStore.viewDeck(`${question.value.deck_id}`); 
  console.log(deck.value); 
  
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
    console.log(openEndedAnswer.value);
  }

 
})
</script>