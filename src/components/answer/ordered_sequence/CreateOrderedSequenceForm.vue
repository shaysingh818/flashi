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
        <GnIconButton label="Add tag" variant="primary" @gn-click="addSequence">
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
  </div>
</template>

<script setup>
import { ref } from "vue";
import FormInputField from "../../../components/shared/forms/FormInputField.vue";
import Tag from "../../../components/shared/Tag.vue";
import OrderedSequenceFormItem from "./OrderedSequenceFormItem.vue";

const newSequence = ref(''); 
const sequences = ref([]); 
const emit = defineEmits(['update:sequence'])

function addSequence() {
  sequences.value.push(newSequence.value); 
  newSequence.value = ''; 
}

function emitSequence() {
  emit('update:sequence', sequences.value); 
}

function removeSequence(index) {
  sequences.value.splice(index, 1);
  emitSequence();  
}
</script>
