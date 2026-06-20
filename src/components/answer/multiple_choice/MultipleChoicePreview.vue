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
      @update:selected="selectedItem = $event; updateAnswer(index)"
      @update:delete="deleteIndex = $event; deleteAnswer()"
    />
    
    <MultipleChoicePreviewItem
      v-if="props.visible === false"
      v-for="(item, index) in items"
      :key="index" 
      :visible="props.visible"
      :index="index"
      :text="item.answer"
      :isSelected="selectedItems.some(s => s.id === item.id)"
      @update:selected="selectedItem = $event; selectAnswer(index)"
    />
  </div> 

</template>

<script setup>
import { ref, onMounted, watch } from 'vue';
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
const selectedItems  = ref([]);
const deleteIndex = ref(null);

watch(() => props.items, () => {
  console.log("Clearing state"); 
  selectedItem.value = null;
  selectedItems.value = [];
}, { immediate: true });


const emit = defineEmits([
  "select:item",
  "select:clear",
  "update:item", 
  "update:add", 
  "update:delete"
]); 


function selectAnswer(index) {

  const localProps = [...props.items];
  const selected = localProps[index];
  const itemIndex = selectedItems.value.findIndex(item => item.id === selected.id);

  if(itemIndex >= 0) {
    selectedItems.value.splice(itemIndex, 1);
  } else {
    selectedItems.value.push(selected);
  }

  console.log(selectedItems.value); 
  emit("select:item", selectedItems.value);  
}


function updateAnswer(index) {
  const localProps = [...props.items];

  const isSelectedItem = selectedItem.value != null; 
  const isNewText = newItem.value != '';
  const existingAnswer = localProps[index].answer;

  if(isSelectedItem && isNewText) {

    localProps[index] = { 
      ...localProps[index], 
      is_correct: selectedItem.value,
      answer: newItem.value
    };
    newItem.value = ''; 
  } else if (isSelectedItem) {
    localProps[index] = { 
      ...localProps[index], 
      is_correct: selectedItem.value,
      answer: existingAnswer
    };
  } else if (isNewText) {
    localProps[index] = { 
      ...localProps[index], 
      answer: newItem.value,
    };
    newItem.value = ''; 
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
