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
  padding: 2px 2px;
  font-size: 13px;
  background-color: #F97316;
  color: white;
  border: none;
  border-radius: 50%;
  height: 30px;
  width: 30px;
  cursor: pointer;
}

.icon-style {
  color: white; 
}

</style>
