<template>

  <div class="flex flex-row gap-2 w-full">
      
      <div class="fw-10">
        <label for="item" v-if="props.visible === true">
            <input 
              type="checkbox" 
              :id="text" 
              :name="text" 
              :checked="props.isSelected"
              @change="emit('update:selected', $event.target.checked)"
            />
        </label>
      </div>

      <div v-if="props.visible === true" class="flex row-center w-full gap-2">
        <GnFormInputField
            class="flex-1"
            @update:model-value="newText = $event; emitText()"
            :model-value="text"
            backgroundColor="var(--app-color-surface)" 
            borderColor="transparent" 
            :disabled="false" 
            textColor="var(--app-color-accent)" />
        <GnIconButton label="Add tag" variant="ghost" @gn-click="deleteMultipleChoiceItem">
            <font-awesome-icon icon="fa-solid fa-x" />
        </GnIconButton>
      </div>
  </div>

  
<div v-if="props.visible === false" class="flex flex-row w-full"> 
   <label for="item">
     <input 
       type="checkbox" 
       :id="text" 
       :name="text" 
       :checked="props.isSelected"
       value="yes" 
       @change="emit('update:selected', $event.target.checked)"
     />
     {{ text }}
   </label>
 </div>  


</template>

<script setup>
import { ref } from 'vue';

const props = defineProps({
  visible: {
    type: Boolean,
    default: () => false,
  },
  index: {
    type: Number,
    required: true,
  },
  text: {
    type: String,
    required: true,
  },    
  isSelected: {
    type: Boolean,
    required: true,
  },

});

const newText = ref(''); 

function emitText(item) {
  emit("update:text", newText.value);
  newText.value = ''; 
}

function deleteMultipleChoiceItem() {
  emit("update:delete", props.index);
}

const emit = defineEmits([
  "update:text", 
  "update:selected", 
  "update:delete"
]);
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
  width: 25px;
  height: 25px;
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
