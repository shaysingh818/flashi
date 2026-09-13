<template>
  <div class="flex flex-col">

    <GnBackBar 
        v-if="deck"
        :title="deck.name"
        subtitle="Last updated 04/02/2026"
        textColor="var(--app-color-text)"
        color="var(--app-color-accent)"
        backgroundColor="var(--app-color-surface)"
        borderColor="var(--app-color-surface)"
        @gn-back="goBack">
        <font-awesome-icon icon="fa-solid fa-arrow-left" /> 
        <template #actions>
          <GnIconButton label="Edit" variant="ghost" size="md" @gn-click="showDropdown">
               <font-awesome-icon icon="fa-solid fa-pen-to-square" />  
          </GnIconButton>
        </template>
    </GnBackBar>
      
    <div class="flex items-center justify-center">
      <QuestionTextPreview
        v-if="question"
        :text="question.raw"
      />
    </div>

    <div class="p-6">
        <GnFileUploadField
            v-model="imageArtifacts"
            @gn-update="onImagesSelected"
            @gn-remove="onImageRemoved"
            :filePicker="selectImages"
            label="Upload optional image associated with text"
            accept="image/*" 
            backgroundColor="var(--app-color-surface)" 
            borderColor="var(--app-color-accent)" 
            color="var(--app-color-accent)" 
            :disabled="false" 
            multiple 
            textColor="var(--app-color-accent)" />
    </div>

    <div class="flex items-center justify-center p-4">
      
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
    :questionId="question.id.toString()"
    :questionText="question.raw"
    :closeFormHandler="showDropdown"
  />
</template>

<script setup>
import { convertFileSrc } from '@tauri-apps/api/core';
import { open } from "@tauri-apps/plugin-dialog";
import { readFile, BaseDirectory } from '@tauri-apps/plugin-fs';
import { onMounted, ref, toRaw, watch, reactive, onBeforeUnmount } from "vue";
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
  imageArtifactStore,
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
const imageArtifacts = ref([]);
const knownImagePaths = ref(new Set()); 
const imageUrls = reactive({}); 
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
const imageIdMap = ref({}); 


async function nextQuestion() {
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
    loadData(question.value.id); 
  }
}

async function loadThumbnail(item) {
  try {
    const fileBuffer = await readFile(`flashi/images/${item}`, {
      baseDir: BaseDirectory.Data,
    });
    return URL.createObjectURL(new Blob([fileBuffer]));
  } catch (err) {
    console.log(`Failed to load thumbnail for ${item}`, err);
  }
}

async function selectImages() {
  const result = await open({
    multiple: true,
    filters: [{ name: "Image Filter", extensions: ["png", "jpeg", "jpg", "webp"] }]
  });

  if (result === null) return [];

  const paths = Array.isArray(result) ? result : [result];
  return paths.map((path) => ({
    name: path.split(/[\\/]/).pop() ?? path,
    path,
    previewUrl: convertFileSrc(path) // optional, for a thumbnail
  }));
}

async function onImagesSelected(items) {
  
  const imageIds = []; 
  const newItems = items.filter((item) => !knownImagePaths.value.has(item.path));
  if (newItems.length === 0) return;

  for (const item of newItems) {
    const fileName = item.name; 
    const localPath = item.path.slice(0, -fileName.length - 1); 
    const [id, result] = await imageArtifactStore.createImageArtifact(
      fileName,
      localPath
    );

    if (id !== -1) {
      imageIds.push(id);
      knownImagePaths.value.add(item.path); 
    } else {
      console.error("Error uploading image: ", item); 
    }
  }

  for (const item of imageIds) {
    const result = await questionStore.addImageArtifactToQuestion(questionId.toString(), item.toString());
    if (result !== "Success") {
      await message(
        'Could not associated one or more images with question',
        { title: 'Recalli', kind: 'info' }
      );
    }
  }

}

async function onImageRemoved(item) {
  const imageId = imageIdMap.value[item.path]; 
  console.log("IMAGE ID TO REMOVE: ", imageId);

  const deleteQuestionImageResult = await questionStore.removeImageArtifactFromQuestion(question.value.id.toString(), imageId.toString());
  console.log("DELETE QUESTION IMAGE RESULT: ", deleteQuestionImageResult); 
  
  const deleteImageResult = await imageArtifactStore.deleteImageArtifact(imageId.toString(), item.path);
  console.log("DELETE IMAGE RESULT: ", deleteImageResult); 
  
  knownImagePaths.value.delete(item.path);

  const url = imageUrls[item.path];
  if (url) {
    URL.revokeObjectURL(url);
    delete imageUrls[item.path];
  }
}

function fileName(item) {
  return item.split('/').pop(); // or however you want to derive display name
}

async function loadData(id) {

  question.value = await questionStore.viewQuestion(`${id}`);
  const answerTypeText = await answerTypeStore.viewAnswerType(`${question.value.answer_type_id}`);
  answerType.value = answerTypeText.name;
  deck.value = await deckStore.viewDeck(`${question.value.deck_id}`); 

  await imageArtifactStore.loadImageArtifacts(`${id}`); 

  const mapped = [];
   for (const item of imageArtifactStore.images) {
     const previewUrl = await loadThumbnail(item.image_path);
     if (previewUrl) imageUrls[item.image_path] = previewUrl;
     mapped.push({
       name: fileName(item.image_path),
       path: item.image_path,
       previewUrl
     });
     imageIdMap.value[fileName(item.image_path)] = item.id;
   }
   imageArtifacts.value = mapped;
   knownImagePaths.value = new Set(mapped.map((item) => item.path));
 
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


    if (updatedOrderedSequences.value.length > 0) { 
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
    } else { 
      console.log("No updates made to ordered sequences"); 
    }

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

function goBack() {
  router.go(-1);
}

onMounted(() => loadData(route.params.id)); 

watch(() => route.params.id, (newId) => {
  if (newId) loadData(newId);
});

onBeforeUnmount(() => {
  for (const url of Object.values(imageUrls)) URL.revokeObjectURL(url);
});

</script>

<style scoped>

.selected-files {
  list-style: none;
  margin: 0;
  padding: 0;
  width: 100%;
}

.file-row {
  display: flex;
  align-items: center;
  gap: 10px;
  padding: 6px 10px;
  width: 100%;
}

.file-name {
  flex-grow: 1;
  min-width: 0; /* lets text-overflow actually kick in inside a flex item */
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  color: #f97316;
  font-size: 12px;
}

.file-thumb {
  flex-shrink: 0;
  border-radius: 4px;
  object-fit: cover;
  height: 50px;
  width: 50px;
}

.file-remove {
  flex-shrink: 0;
  background-color: rgb(28 25 23);
  color: #f97316;
  border: none;
  border-radius: 3px;
}

</style>