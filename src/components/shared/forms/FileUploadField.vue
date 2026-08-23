<template>
    <input 
        @click="selectImages" 
        type="button" 
        :id="inputId" 
        :name="inputId" >
            
    <label :for="inputId">{{ fieldLabel }}</label>

    <ul v-if="selectedPaths.length" class="selected-files">
       <li v-for="path in selectedPaths" :key="path">
         <img :src="previewSrc(path)" alt="" width="48" height="48">
         <span>{{ fileName(path) }}</span>
       </li>
     </ul>

</template>

<script setup>
import { ref } from 'vue';
import { open } from '@tauri-apps/plugin-dialog';
import { convertFileSrc } from '@tauri-apps/api/core';


const props = defineProps({
  fieldLabel: {
      type: String,
      required: true,
  },    
  fieldLabel: {
    type: Boolean,
    default: true,
  },
});

const emit = defineEmits(['update:modelValue'])
const inputId = 'file-upload'; 
const selectedPaths = ref([]); 

function fileName(path) {
  return path.split(/[\\/]/).pop();
}

function previewSrc(path) {
  return convertFileSrc(path);
}

async function selectImages() {
  const result = await open({
    multiple: props.multiple,
    filters: [
      {
        name: 'Image Filter',
        extensions: ['png', 'jpeg', 'jpg', 'webp'],
      },
    ],
  });

  if (result === null) {
    return;
  }

  selectedPaths.value = Array.isArray(result) ? result : [result];
  emit('update:modelValue', selectedPaths.value);
}
</script>

<style scoped>

input {
  border-radius: 3px;
  width: 100%;
  height: 100%;
  background-color: rgb(28 25 23);
  color: #f97316;
  border: none;
  padding-left: 5px;
}

label {
  background-color: rgb(28 25 23);
  color: #f97316;
  padding: 12px 24px;
  border-radius: 10px;
  font-size: 14px;
  cursor: pointer;
  font-weight: bold;
}

.selected-files {
  list-style: none;
  padding: 0;
  display: flex;
  gap: 8px;
  flex-wrap: wrap;
}
.selected-files li {
  display: flex;
  flex-direction: column;
  align-items: center;
  font-size: 12px;
}
.selected-files img {
  object-fit: cover;
  border-radius: 4px;
}

</style>
