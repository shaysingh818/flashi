<template>

  <div class="flex flex-col gap-2">
    
      <GnFormTextField
          v-model="text"
          backgroundColor="var(--app-color-surface)" 
          borderColor="transparent" 
          :disabled="false" 
          placeholder="Write something…" 
          :rows="4"        
          label="Enter question answer text"
          textColor="var(--app-color-accent)" />
    
    <div class="flex flex-row items-end gap-2">
        <GnFormInputField
            class="flex-1"
            v-model="newKeyword"
            backgroundColor="var(--app-color-surface)" 
            borderColor="transparent" 
            :disabled="false" 
            placeholder="Add answer keyword…" 
            label="Add answer keyword"
            textColor="var(--app-color-accent)" />
        <GnIconButton label="Add tag" variant="primary" @gn-click="addKeyword">
            <font-awesome-icon icon="fa-solid fa-plus" />
        </GnIconButton>
    </div> 

    <div class="flex flex-row flex-wrap justify-center gap-2 p-4" style="min-height: 15px;">
      <GnTag
        v-for="(item, index) in keywords"
        :key="index"
        :label="item"
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
