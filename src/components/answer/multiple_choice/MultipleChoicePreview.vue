<template>
  <div class="flex col-center w-100 gap-2 p-2">

    <div v-if="visible === true" class="flex row-center gap-2 w-85" style="height: 35px;">
      <FormInputField
        placeholder="Add more multiple choice items."
        @update:text="newItem = $event"
      />

      <font-awesome-icon 
        icon="fa-solid fa-plus" 
        style="color: #f97316" 
        @click="addMultipleChoice" 
      />
    </div>

    <MultipleChoicePreviewItem
      v-if="props.visible === true"
      v-for="(item, index) in items"
      :key="index"
      :visible="props.visible"
      :index="index"
      :text="item.answer"
      :isSelected="item.is_correct"
      @update:text="updateAnswerText(index, $event)"
      @update:selected="updateAnswerSelected(index, $event)"
      @update:delete="deleteIndex = $event; deleteAnswer()"
    />
    
    <MultipleChoicePreviewItem
      v-if="props.visible === false"
      v-for="(item, index) in items"
      :key="index"
      :text="item.answer"
      :isSelected="false"
    />
  </div> 

</template>

<script setup>
import { ref } from 'vue';
import FormInputField from '../../shared/forms/FormInputField.vue';
import MultipleChoicePreviewItem from './MultipleChoicePreviewItem.vue';
import { multipleChoiceStore, multipleChoiceAnswerStore } from "../../../api";

const props = defineProps({  
  visible: {
    type: Boolean,
    required: true,
  },
  items: {
    type: Array,
    required: true,
  },
});

const newItem = ref('');
const deleteIndex = ref(null);
const changedItems = ref({});
const emit = defineEmits([
  "update:item", 
  "update:add", 
  "update:delete"
]); 

function updateAnswerText(index, text) {
  const localProps = [...props.items];
  localProps[index] = { ...localProps[index], answer: text };
  changedItems.value[index] = { ...localProps[index] };
  emit("update:item", localProps, changedItems);
}

function updateAnswerSelected(index, selected) {
  const localProps = [...props.items];
  localProps[index] = { ...localProps[index], is_correct: selected };
  changedItems.value[index] = { ...localProps[index] };
  emit("update:item", localProps, changedItems);
}

function addMultipleChoice() {
  emit("update:add", newItem.value);
  newItem.value = ''; 
}

function deleteAnswer() {
  emit("update:delete", deleteIndex.value);
  deleteIndex.value = null; 
}
</script>
