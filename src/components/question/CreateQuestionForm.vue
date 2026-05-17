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
import { openEndedStore } from "../../api/OpenEndedAnswer";
import { orderedSequenceStore } from "../../api/OrderedSequenceAnswer";
import { multipleChoiceStore } from "../../api/MutlipleChoiceAnswer";
import { AnswerType, answerTypeStore } from "../../api/AnswerType";
import FormInputField from "../../components/shared/forms/FormInputField.vue";
import FormTextField from "../../components/shared/forms/FormTextField.vue";
import PrimaryButton from "../../components/shared/buttons/PrimaryButton.vue";
import SecondaryButton from "../../components/shared/buttons/SecondaryButton.vue";
import Tag from "../shared/Tag.vue";
import DropDownFormField from "../shared/forms/DropDownFormField.vue";
import CreateMultipleChoiceForm from "../answer/multiple_choice/CreateMultipleChoiceForm.vue";
import CreateOpenEndedForm from "../answer/open_ended/CreateOpenEndedForm.vue";
import CreateOrderedSequenceForm from "../answer/ordered_sequence/CreateOrderedSequenceForm.vue";


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


function submitMultipleChoice() {
  const answerTypeId = retrieveAnswerTypeId('Multiple Choice'); 
}

function submitOpenEnded() {
  const answerTypeId = retrieveAnswerTypeId('Open Ended'); 
}

function submitOrderedSequence() {
  const answerTypeId = retrieveAnswerTypeId('Ordered Sequence'); 
}

function submitQuestion() {
  
  console.log(text.value); 

  const answerId = 0;
  
  if (selectedOption.value === 'Multiple Choice') {
    submitMultipleChoice();
  }

  if (selectedOption.value === 'Open Ended') {
    submitOpenEnded();
  }
  
  if (selectedOption.value === 'Ordered Sequence') {    
    submitOrderedSequence();
  }
}

</script>
