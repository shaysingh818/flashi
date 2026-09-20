<template>
    <div class="flex items-center gap-2 w-3/4">
    <div class="flex items-center gap-2 basis-3/4">
        <p class="number-index">{{ index }}.</p>
        <p class="indice-content">{{ content }}</p>
    </div>
    <div class="flex items-center justify-center basis-1/4">
        <font-awesome-icon icon="fa-solid fa-x" @click.prevent="emit('remove', props.index - 1)" />
    </div>
    </div>

</template>

<script setup>
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
  font-size: 15px; 
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