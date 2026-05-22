<template>
  <div class="flex col-center gap-4 w-100">
    <div class="flex w-75" style="height: 150px;">
      <FormTextField v-model:text="text" description="Deck Question" />
    </div>

    <div class="flex centered-all w-75">
      <DropDownFormField
        title="choose option"
        @update:title="selectedOption = $event;"
        :options="answerTypeNames"
      />
    </div>

    <CreateMultipleChoiceForm 
      v-if="selectedOption == 'Multiple Choice'" 
      @update:items="multipleChoiceAnswers = $event"
    />

    <CreateOpenEndedForm 
      v-if="selectedOption == 'Open Ended'" 
      @update:text="openEndedText = $event" 
      @update:keywords="openEndedKeywords = $event"
    />

    <CreateOrderedSequenceForm 
      v-if="selectedOption == 'Ordered Sequence'"
      @update:sequence="orderedSequences = $event" 
    />

    
    <br />
    <div class="flex row-center gap-4" style="height: 35px">
      <SecondaryButton :onClickHandler="goBack" text="Discard" />
      <PrimaryButton :onClickHandler="submitQuestion" text="Submit" />
    </div>
  </div>
</template>

<script setup>
import { ref } from "vue";
import { onMounted } from "vue";
import { useRoute, useRouter } from "vue-router";
import FormInputField from "../../components/shared/forms/FormInputField.vue";
import FormTextField from "../../components/shared/forms/FormTextField.vue";
import PrimaryButton from "../../components/shared/buttons/PrimaryButton.vue";
import SecondaryButton from "../../components/shared/buttons/SecondaryButton.vue";
import Tag from "../shared/Tag.vue";
import DropDownFormField from "../shared/forms/DropDownFormField.vue";
import CreateMultipleChoiceForm from "../answer/multiple_choice/CreateMultipleChoiceForm.vue";
import CreateOpenEndedForm from "../answer/open_ended/CreateOpenEndedForm.vue";
import CreateOrderedSequenceForm from "../answer/ordered_sequence/CreateOrderedSequenceForm.vue";

import { 
  questionStore,
  AnswerType, answerTypeStore,
  opendEndedStore,
  orderedSequenceStore,
  multipleChoiceStore,
  multipleChoiceAnswerStore
} from "../../api";

const props = defineProps({
  deckId: {
    type: String,
    required: true,
  },
  questionText: {
    type: String,
    required: true,
  },
});

const router = useRouter(); 
const text = ref(''); 
const selectedOption = ref('');
const orderedSequences = ref([]);
const openEndedText = ref(''); 
const openEndedKeywords = ref([]); 
const multipleChoiceAnswers = ref([]); 
const answerTypeNames = ref([]); 

onMounted(async () => {
  await answerTypeStore.loadAnswerTypes();
  const names = answerTypeStore.answerTypes.map(item => item.name);
  answerTypeNames.value = names;
})

function goBack(){
  router.go(-1); 
}

function retrieveAnswerTypeId(key) {
  for (const item of answerTypeStore.answerTypes) {
    if (item.name === key) {
      return item.answer_type_id
    }
  }
}

async function submitMultipleChoice(multipleChoiceAnswers) { 
  const answer = await multipleChoiceStore.createMultipleChoice();
  const answerId = answer.answer_id; 
  for (const item of multipleChoiceAnswers) {
    await multipleChoiceAnswerStore.createMultipleChoiceAnswer(
      `${answerId}`,
      item.text,
      '',
      item.selected
    );
  }
  return answerId;
}

async function submitOpenEnded(answerText, explanation, keywords) {
  const answer = await openEndedStore.createOpenEndedAnswer(
    answerText,
    explanation,
    keywords
  );
  return answer.answer_id; 
}

async function submitOrderedSequence(sequence, explanation) {
  const answer = await orderedSequenceStore.createOrderedSequenceAnswer(
    sequence,
    explanation
  ); 
  return answer.answer_id;
}

async function submitQuestion() {
  
  const deckId = props.deckId; 
  
  if (selectedOption.value === 'Multiple Choice') {    
    const answerTypeId = retrieveAnswerTypeId('Multiple Choice');
    const answerId = await submitMultipleChoice(multipleChoiceAnswers.value);     
    const insertResult = await questionStore.createQuestion(
      deckId,
      answerTypeId.toString(),
      answerId.toString(),
      text.value
    );
    console.log(insertResult); 
  }

  if (selectedOption.value === 'Open Ended') { 
    const answerTypeId = retrieveAnswerTypeId('Open Ended');
    const answerId = await submitOpenEnded(
      openEndedText.value,
      '',
      openEndedKeywords.value
    );

    const insertResult = await questionStore.createQuestion(
      deckId,
      answerTypeId.toString(),
      answerId.toString(),
      text.value
    );
    console.log(insertResult); 

  }
  
  if (selectedOption.value === 'Ordered Sequence') { 
    const answerTypeId = retrieveAnswerTypeId('Ordered Sequence');
    const answerId = await submitOrderedSequence(
      orderedSequences.value,
      ''
    );
    
    const insertResult = await questionStore.createQuestion(
      deckId,
      answerTypeId.toString(),
      answerId.toString(),
      text.value
    );
    console.log(insertResult); 
  }
}

</script>
