<template>

<div class="flex flex-col gap-4 p-6 w-full">
  
  <GnFormTextField
      v-if="props.visible === true"
      :model-value="props.placeholderText"
      @update:model-value="newText = $event; emitQuestionText()"
      @gn-update="emitQuestionText"
      backgroundColor="var(--app-color-surface)" 
      borderColor="transparent" 
      :disabled="false" 
      :rows="4"        
      textColor="var(--app-color-accent)" />

  <GnFormTextField
      v-if="props.visible === false"
      placeholder="Enter your answer"
      @update:model-value="newText = $event; emitQuestionText()"
      @gn-update="emitQuestionText"
      backgroundColor="var(--app-color-surface)" 
      borderColor="transparent" 
      :disabled="false" 
      :rows="4"        
      textColor="var(--app-color-accent)" />

  <div v-if="visible === true" class="flex flex-row gap-2">
    <GnFormInputField
        class="flex-1"
        @update:model-value="newKeyword = $event; emitText()"
        backgroundColor="var(--app-color-surface)" 
        borderColor="transparent"
        placeholder="Add more keywords for open ended answer, you can delete keywords by double clicking on them."
        :disabled="false" 
        textColor="var(--app-color-accent)" />
    <GnIconButton label="Add tag" variant="ghost" @gn-click="addKeyword">
        <font-awesome-icon icon="fa-solid fa-plus" />
    </GnIconButton>
  </div>

  <div class="flex flex-row flex-wrap justify-center gap-2 p-4"> 
    <GnTag
        v-if="visible === true"
        v-for="(item, index) in keywords"
        :key="index"
        :label="item"
        @gn-dblclick="removeKeyword(index)"
    />
  </div>

  <GnFormTextField
      v-if="props.visible === true"
      :model-value="props.explanation"
      @update:model-value="emit('update:explanation', $event)"
      backgroundColor="var(--app-color-surface)" 
      borderColor="transparent" 
      :disabled="false" 
      placeholder="Write explanation for question answer…" 
      :rows="4"        
      label="Question explanation"
      textColor="var(--app-color-accent)" />
</div>


</template>

<script setup>
import { ref } from 'vue';
import { GnTag } from '@graphyne/web-components';

const props = defineProps({
  visible: {
    type: Boolean,
    required: true
  },
  placeholderText: {
    type: String,
    required: true,
  },
  keywords: {
    type: Array,
    required: true,
  },
  explanation: {
    type: String,
    default: '',
  },
});

const newKeyword = ref('');
const newText = ref('');
const emit = defineEmits(["update:keywords", "update:text", "update:explanation"]);

function removeKeyword(index) {
  props.keywords.splice(index, 1);
  emit('update:keywords', props.keywords);
}

function addKeyword() {
  props.keywords.push(newKeyword.value.trim());
  newKeyword.value = '';
}

function emitKeywords() {
  emit('update:keywords', props.keywords);
}

function emitQuestionText() {
  emit('update:text', newText.value);
}
</script>
