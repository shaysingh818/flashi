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
      @update:text="newItem = $event; updateAnswer(index)"
      @update:selected="selectedItem = $event; updateAnswer(index, $event)"
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
const selectedItem = ref(null);
const deleteIndex = ref(null);
const emit = defineEmits([
  "update:item", 
  "update:add", 
  "update:delete"
]); 

function updateAnswer(index) {
  const localProps = [...props.items];

  const isSelectedItem = selectedItem.value != null; 
  const isNewText = newItem.value != null;

  if(isSelectedItem && isNewText) {
    localProps[index] = { 
      ...localProps[index], 
      is_correct: selectedItem.value,
      answer: newItem.value
    };
  } else if (isSelectedItem) {
    localProps[index] = { 
      ...localProps[index], 
      is_correct: selectedItem.value,
    };
  } else if (isNewText) {
    localProps[index] = { 
      ...localProps[index], 
      answer: newItem.value,
    };
  }

  emit("update:item", localProps[index]);
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
