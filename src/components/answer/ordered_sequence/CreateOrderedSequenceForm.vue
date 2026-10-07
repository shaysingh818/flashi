<template>
  <div class="flex flex-col gap-4">
      
    <div class="flex flex-row items-end gap-2">
        <GnFormInputField
            class="flex-1"
            v-model="newSequence"
            @gn-update="emitSequence"
            backgroundColor="var(--app-color-surface)" 
            borderColor="transparent" 
            :disabled="false" 
            placeholder="Add ordered sequence item" 
            label="Add ordered sequence item"
            textColor="var(--app-color-accent)" />
        <GnIconButton label="Add tag" variant="ghost" size="sm" @gn-click="addSequence">
            <font-awesome-icon icon="fa-solid fa-plus" />
        </GnIconButton>
    </div> 
      
    <div class="flex flex-col gap-2 p-4">
      <OrderedSequenceFormItem
        v-for="(item , index) in sequences"
        :key="index"
        :index="index+1"
        :content="item"
        @remove="removeSequence"
      />
    </div>

    <GnFormTextField
        v-model="explanation"
        @gn-update="emitQuestionExplanation"
        backgroundColor="var(--app-color-surface)" 
        borderColor="transparent" 
        :disabled="false" 
        placeholder="Write explanation for question answer…" 
        :rows="4"        
        label="Enter question explanation"
        textColor="var(--app-color-accent)" />

  </div>
</template>

<script setup>
import { ref } from "vue";
import OrderedSequenceFormItem from "./OrderedSequenceFormItem.vue";

const newSequence = ref(''); 
const sequences = ref([]);
const explanation = ref(''); 
const emit = defineEmits(['update:sequence', 'update:explanation'])

function addSequence() {
  sequences.value.push(newSequence.value); 
  newSequence.value = ''; 
}

function emitSequence() {
  emit('update:sequence', sequences.value); 
}

function emitQuestionExplanation() {
  emit('update:explanation', explanation.value);
}

function removeSequence(index) {
  sequences.value.splice(index, 1);
  emitSequence();  
}
</script>
