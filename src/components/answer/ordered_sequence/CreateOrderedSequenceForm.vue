<template>
  <div class="flex col-center gap-4 w-75">
    <div class="flex row-center gap-2 w-100" style="height: 35px">
      <FormInputField
        placeholder="Add item for ordered sequence answer"
        @update:text="newSequence = $event; emitSequence()"
      />
      <font-awesome-icon icon="fa-solid fa-plus" style="color: #f97316" @click="addSequence" />
    </div>

    <div class="flex col-center w-100 p-2 rounded-2">
      <OrderedSequenceFormItem
        v-for="(item , index) in sequences"
        :key="index"
        :index="index+1"
        :content="item"
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
</script>
