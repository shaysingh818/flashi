<template>
  <button @click="handleClick"> 
    <font-awesome-icon :icon="`${icon}`" class="icon-style" />
  </button>
</template>

<script setup>

const props = defineProps({
  icon: {
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
  position: fixed;
  bottom: 20px;
  right: 20px;    
  padding: 10px 20px;
  font-size: 16px;
  background-color: #F97316;
  color: white;
  border: none;
  border-radius: 50%;
  height: 70px;
  width: 70px;
  cursor: pointer;
}

.icon-style {
  color: white; 
}

</style>
