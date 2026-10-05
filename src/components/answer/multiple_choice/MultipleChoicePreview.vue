<template>
  <div class="flex flex-col gap-2 w-full p-6">

    <div v-if="visible === true" class="flex flex-row items-end gap-2 p-2" style="height: 35px;">
      <GnFormInputField
          class="flex-1"
          v-model="newItem"
          backgroundColor="var(--app-color-surface)" 
          borderColor="transparent" 
          :disabled="false" 
          placeholder="Add multiple choice item" 
          textColor="var(--app-color-accent)" />
      <GnIconButton label="Add tag" variant="ghost" size="sm" @gn-click="addMultipleChoice">
          <font-awesome-icon icon="fa-solid fa-plus" />
      </GnIconButton>
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
      @update:explanation="explanation = $event; updateExplanation()" />
    
    <MultipleChoicePreviewItem
      v-if="props.visible === false"
      v-for="(item, index) in items"
      :key="index" 
      :visible="props.visible"
      :index="index"
      :text="item.answer"
      :isSelected="selectedItems.some(s => s.id === item.id)"
      @update:selected="selectedItem = $event; selectAnswer(index)" />
  </div> 

</template>

<script setup>
import { ref, watch } from 'vue';
import MultipleChoicePreviewItem from './MultipleChoicePreviewItem.vue';

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
const explanation = ref('');

watch(() => props.items, () => {
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
  const isNewExplanation = explanation.value != '';
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
