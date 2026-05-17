<template>

  <div class="flex col-center w-75 gap-2">
    
    <div class="flex p-4 w-100">
      <FormTextField description="Type open ended answers" @update:text="text = $event; emitQuestionText()" />
    </div>
    
    <div class="flex row-center gap-2 w-100" style="height: 35px;">
      <FormInputField
        placeholder="Add keyword for open ended answer"
        @update:text="newKeyword = $event; emitKeywords()"
      />
      <font-awesome-icon icon="fa-solid fa-plus" style="color: #f97316" @click="addKeyword" />
    </div>

    <div class="flex row-center align-left wrap gap-2 p-2">
      <Tag
        v-for="(item, index) in keywords"
        :key="index"
        :title="item"
        height="10px"
      />
    </div>
  </div>

</template>

<script setup>
import { ref } from "vue";
import FormInputField from "../../../components/shared/forms/FormInputField.vue";
import FormTextField from "../../../components/shared/forms/FormTextField.vue";
import Tag from "../../../components/shared/Tag.vue";

const text = ref(''); 
const newKeyword = ref('');
const keywords = ref([]);
const emit = defineEmits(['update:text', 'update:keywords']); 

function addKeyword() {
  keywords.value.push(newKeyword.value); 
  newKeyword.value = '';
}

function emitKeywords() {
  emit('update:keywords', keywords.value);
}

function emitQuestionText() {
  emit('update:text', text.value);
}

</script>
