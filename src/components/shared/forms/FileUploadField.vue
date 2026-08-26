<template>

    <div class="flex col-center w-100">

        <div class="flex centered-all w-75" style="height: 50px;">
            <input 
                @click="selectImages" 
                type="button" 
                :id="inputId" 
                :name="inputId"
                value="Upload image associated with question">
        </div>
        <br /> 

        <div class="flex col-center w-100">
            <div class="flex row-center w-75">
                <ul v-if="selectedPaths.length" class="selected-files">
                    <li v-for="path in selectedPaths" :key="path" class="file-row">
                        <img :src="previewSrc(path)" alt="" width="40" height="40" class="file-thumb">
                        <span class="file-name">{{ fileName(path) }}</span>
                        <button type="button" class="file-remove" @click="removePath(path)">✕</button>
                    </li>
                </ul>
            </div>
        </div>

    </div>

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
  multiple: {
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
  const src = convertFileSrc(path); 
  return src;
}

function removePath(path) {
  selectedPaths.value = selectedPaths.value.filter(p => p !== path);
  emit('update:modelValue', selectedPaths.value);
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

  console.log("Is multiple: ", props.multiple); 
  console.log('raw result:', result, Array.isArray(result));

  if (result === null) {
    return;
  }

  const newPaths = Array.isArray(result) ? result : [result];
  selectedPaths.value = [...new Set([...selectedPaths.value, ...newPaths])]; 
  console.log(selectedPaths.value); 
  emit('update:modelValue', selectedPaths.value);
}


</script>

<style scoped>

input {
  border-radius: 5px;
  width: 75%;
  height: 100%;
  background-color: rgb(28 25 23);
  color: #f97316;
  border: none;
  padding-left: 5px;
  font-weight: bold;
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
  margin: 0;
  padding: 0;
  width: 100%;
}

.file-row {
  display: flex;
  align-items: center;
  gap: 10px;
  padding: 6px 10px;
  width: 100%;
}

.file-name {
  flex-grow: 1;
  min-width: 0; /* lets text-overflow actually kick in inside a flex item */
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  color: #f97316;
  font-size: 12px;
}

.file-thumb {
  flex-shrink: 0;
  border-radius: 4px;
  object-fit: cover;
}

.file-remove {
  flex-shrink: 0;
  background-color: rgb(28 25 23);
  color: #f97316;
  border: none;
  border-radius: 3px;
}

</style>
