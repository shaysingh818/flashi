<template>

<div class="flex col-center w-100">
  <div class="flex w-85 p-4" style="height: 100px;">
    
   <FormTextField   
    v-if="props.visible === true"
    :description="props.placeholderText"
    :text="props.placeholderText"
    @update:text="newText = $event; emitQuestionText()"
    />

    <FormTextField   
     v-if="visible === false"
     description="Enter your answer"
    :text="props.placeholderText"
    />
    
  </div>

  <div v-if="visible === true" class="flex row-center gap-2 w-85" style="height: 35px;">
    <FormInputField
      placeholder="Update keyword for open ended answer"
      @update:text="newKeyword = $event; emitKeywords()"
    />
    <font-awesome-icon icon="fa-solid fa-plus" style="color: #f97316" @click="addKeyword" />
  </div>

  <div class="flex row-center align-left wrap gap-2 p-4 w-85">
    <Tag
      v-if="visible === true"
      v-for="(item, index) in keywords"
      :key="index"
      :title="item"
      height="10px"
    />
  </div>
</div>


</template>

<script setup>
import { ref } from 'vue';
import Tag from '../../shared/Tag.vue';
import FormInputField from '../../shared/forms/FormInputField.vue';
import FormTextField from '../../../components/shared/forms/FormTextField.vue';

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
  }
});

const newKeyword = ref('');
const newText = ref('');
const emit = defineEmits(["update:keywords", "update:text"]);

function addKeyword() {
  console.log("is this being called");
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
