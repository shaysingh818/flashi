<template>
  <div class="flex flex-col gap-4 p-6">

    <GnFormTextField
        v-model="text"
        backgroundColor="var(--app-color-surface)" 
        borderColor="transparent" 
        :disabled="false" 
        placeholder="Question text" 
        :rows="4"        
        label="Enter question text"
        textColor="var(--app-color-accent)" />

    
      <GnFormDropDownField >
          
      </GnFormDropDownField>

    <div class="flex centered-all w-75 oultine">
        <FileUploadField
            fieldLabel="Upload Images (Optional)"
            :multiple="true"
            @update:modelValue="onImagesSelected"
        />
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
      <SecondaryButton @click="goBack" text="Discard" />
      <PrimaryButton @click="submitQuestion" text="Submit" />
    </div>
  </div>
</template>

<script setup>
import { ref } from "vue";
import { onMounted } from "vue";
import { useRoute, useRouter } from "vue-router";
import { message } from '@tauri-apps/plugin-dialog';
import CreateMultipleChoiceForm from "../answer/multiple_choice/CreateMultipleChoiceForm.vue";
import CreateOpenEndedForm from "../answer/open_ended/CreateOpenEndedForm.vue";
import CreateOrderedSequenceForm from "../answer/ordered_sequence/CreateOrderedSequenceForm.vue";

import {
  PrimaryButton,
  SecondaryButton,  
  FormInputField,
  FormTextField,
  DropDownFormField,
  FileUploadField,
  Tag
} from "../../components/shared";

import { 
  questionStore,
  imageArtifactStore,
  answerTypeStore,
  openEndedStore,
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
const selectedPaths = ref([]);
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

async function onImagesSelected(paths) {
  selectedPaths.value = paths;
}

function fileName(path) {
  return path.split(/[\\/]/).pop();
}


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

async function submitQuestionImages(selectedImagePaths) {
  let imageArtifactIds = []; 
  for (const item of selectedImagePaths) {
    const fileName = item.split('/').pop(); 
    const localPath = item.slice(0, -fileName.length - 1);
    const [id, result] = await imageArtifactStore.createImageArtifact(
      fileName,
      localPath
    ); 
    if (id !== -1) {
      imageArtifactIds.push(id); 
    } else {
      console.error("Error uploading image: ", item); 
    }
  }
  return imageArtifactIds
}

async function submitMultipleChoice(multipleChoiceAnswers) { 
  const answer = await multipleChoiceStore.createMultipleChoice();
  const answerId = answer.answer_id; 
  console.log(multipleChoiceAnswers); 
  if (multipleChoiceAnswers.length === 0) {
    return -1;
  }
  
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
  if (answerText.length === 0 || keywords.length === 0) {
    return -1;
  }
  
  return answer.answer_id; 
}

async function submitOrderedSequence(sequence, explanation) {
  const answer = await orderedSequenceStore.createOrderedSequenceAnswer(
    sequence,
    explanation
  );  
  if (sequence.length === 0) {
    return -1;
  }
  return answer.answer_id;
}

async function submitQuestion() {
  
  const deckId = props.deckId; 
  let questionInsertId = -1;
  let questionInsertResult = ""; 
  
  if (selectedOption.value === 'Multiple Choice') {    
    const answerTypeId = retrieveAnswerTypeId('Multiple Choice');
    const answerId = await submitMultipleChoice(multipleChoiceAnswers.value);     
    if (answerId !== -1) { 
      [questionInsertId, questionInsertResult] = await questionStore.createQuestion(
        deckId,
        answerTypeId.toString(),
        answerId.toString(),
        text.value
      );
      if (questionInsertResult === "Success") { 
        await message(
          'Created multiple choice question for deck',
          { title: 'Recalli', kind: 'info' }
        );
      } else { 
        await message(
          `Could not create multiple choice question: ${questionInsertResult}`,
          { title: 'Recalli', kind: 'info' }
        );
      }
    } else {      
      await message(
        'Could not create multiple choice question, please fill out all required fields',
        { title: 'Recalli', kind: 'info' }
      );
    }
  } else if (selectedOption.value === 'Open Ended') { 
    const answerTypeId = retrieveAnswerTypeId('Open Ended');
    const answerId = await submitOpenEnded(
      openEndedText.value,
      '',
      openEndedKeywords.value
    );
    if (answerId !== -1) { 
      [questionInsertId, questionInsertResult] = await questionStore.createQuestion(
        deckId,
        answerTypeId.toString(),
        answerId.toString(),
        text.value
      );    
      if (questionInsertResult === "Success") { 
        await message(
          'Created open ended question for deck',
          { title: 'Recalli', kind: 'info' }
        );
      } else { 
        await message(          
          `Could not create open ended question: ${questionInsertResult}`,
          { title: 'Recalli', kind: 'info' }
        );
      }
    } else {      
      await message(
        'Could not create open ended question, please fill out all required fields',
        { title: 'Recalli', kind: 'info' }
      );
    }

  } else if (selectedOption.value === 'Ordered Sequence') { 
    const answerTypeId = retrieveAnswerTypeId('Ordered Sequence');
    const answerId = await submitOrderedSequence(
      orderedSequences.value,
      ''
    );
    if (answerId !== -1) { 
      [questionInsertId, questionInsertResult] = await questionStore.createQuestion(
        deckId,
        answerTypeId.toString(),
        answerId.toString(),
        text.value
      );
      if (questionInsertResult === "Success") { 
        await message(
          'Created ordered sequence question for deck',
          { title: 'Recalli', kind: 'info' }
        );
      } else { 
        await message(          
          `Could not create ordered sequence question: ${insertResult}`,
          { title: 'Recalli', kind: 'info' }
        );
      }
    } else {      
      await message(
        'Could not create ordered sequence question, please fill out all required fields',
        { title: 'Recalli', kind: 'info' }
      );
    }
 
  } else { 
    await message(
      'Could not create question, select question type and fill out required fields',
      { title: 'Recalli', kind: 'info' }
    );
  }

  if (selectedPaths.value.length > 0) {
    const imageIds = await submitQuestionImages(selectedPaths.value);
    for (const item of imageIds) {
      const result = await questionStore.addImageArtifactToQuestion(questionInsertId.toString(), item.toString());
      if (result !== "Success") {
        await message(
          'Could not associated one or more images with question',
          { title: 'Recalli', kind: 'info' }
        );
      }
    }
  }
  
}

</script>
