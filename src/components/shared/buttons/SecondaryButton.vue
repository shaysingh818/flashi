<template>
  <button @click="handleClick" type="button">{{ text }} </button> 
</template>

<script setup>
const props = defineProps({
  text: {
    type: String,
    required: true,
  },
});

const emit = defineEmits(['click']);

async function handleClick() {
  emit('click');
}

</script>

<style scoped>

button {
  border-radius: 5px;
  width: 100px;
  height: 100%; 
  color: #F97316;
  background-color: inherit;  
  border: 2px solid #F97316;
  font-weight: bold; 
}

button:focus {
  outline: none; /* removes it entirely */
}

button:focus-visible {
  outline: 2px solid blue; /* Custom accessible ring */
  outline-offset: 2px;
}

</style>
