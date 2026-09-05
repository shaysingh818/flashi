<template>
  <div class="flex row-center gap-2 w-75">
    <div class="flex row align-left fw-75 gap-2">
      <p class="number-index">{{ index }}.</p>
      <p class="indice-content">{{ content }}</p>
    </div>
    <div class="flex row-center fw-25">
      <font-awesome-icon icon="fa-solid fa-x" @click.prevent="emit('remove', props.index - 1)" />
    </div>
  </div>

</template>

<script setup>
import { ref } from "vue";

const props = defineProps({
  index: {
    type: String,
    required: true,
  },
  content: {
    type: String,
    required: true,
  },
});

const emit = defineEmits(['remove']); 
</script>

<style scoped>

.number-index {
  color:  #f97316;
  font-size: 12px; 
}


.indice-content {
  color: #b5b3b3;
  font-size: 12px; 
}

svg {
  color: #f97316;
  width: 15px; 
  height: 15px; 
}

</style>