<template>
  <div class="search-container">
    <FormInputField v-model:text="text" :placeholder="placeholder" />
    <PrimaryButton @click="onClick" text="Search" />
  </div>
</template>

<script setup>
import { ref } from 'vue';
import FormInputField from "./FormInputField.vue";
import PrimaryButton from "../buttons/PrimaryButton.vue";

const props = defineProps({
  placeholder: {
    type: String,
    default: "Search",
  },
});

const emit = defineEmits(['search']);
const text = ref('');

function onClick() {
  emit('search', text.value);
}
</script>

<style scoped>
.search-container {
  display: flex;
  flex-direction: row;
  justify-content: center;
  align-items: center;
  height: 40px;
  width: 90%;
  gap: 0.5rem;
}
</style>
