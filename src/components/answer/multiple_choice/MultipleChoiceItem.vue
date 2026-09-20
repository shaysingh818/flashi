<template>
  <label for="item">
    <input 
      type="checkbox"
      :text="choiceText"
      :id="text" 
      :name="text"
      :checked="selected"
      @change="emit('update:items', $event.target.checked)"/>
    {{ text }}
    <font-awesome-icon icon="fa-solid fa-x" @click.prevent="emit('remove')" />
  </label>
</template>

<script setup>
import { ref } from "vue";

const props = defineProps({  
  text: {
    type: String,
    required: true,
  },
  selected: {
    type: Boolean,
    default: false,
  }
});

const emit = defineEmits(['update:items', 'remove']);
const choiceText = ref('');

</script>

<style scoped>
label {
  display: flex;
  padding: 10px;
  color: #b5b3b3;
  align-items: center;
  justify-content: left;
  width: 100%;
  gap: 1rem;
  font-size: 12px;
}

input[type="checkbox"] {
  appearance: none;
  width: 20px;
  height: 20px;
  background-color: rgb(28 25 23);
  border-radius: 2px;
}

input[type="checkbox"]:checked {
  background-color: #f97316;
  border-color: #007bff;
}

svg {
  color: #f97316;
}
</style>
