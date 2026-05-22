<template>
  <div class="flex col-center gap-2 w-85 p-4 ml-2">
    <MultipleChoicePreviewItem
      v-if="props.visible === true"
      v-for="(item, index) in items"
      :key="index"
      :text="item.answer"
      :isSelected="item.is_correct"
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
})
</script>