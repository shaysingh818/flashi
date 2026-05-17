<template>
  <div class="flex col-center gap-4">

    <div class="flex w-75">
      <SettingsToggle text="Allow random generation of multiple choice answers. This will add multiple choice items that come from other answers to other questions in the deck." />
    </div>
    
    <div class="flex row-center gap-2 w-75" style="height: 40px">
      <FormInputField
        placeholder="Add multiple choice answer"
        @update:text="newItem = $event"
      />
      <font-awesome-icon icon="fa-solid fa-plus" @click="addItem" />
    </div>

    <div class="flex col-center p-2 w-75">
      <MultipleChoiceItem
        v-for="(item, index) in items"
        :key="index"
        :text="item.text"
        :selected="item.selected"
        @update:items="item.selected = $event; emitItems()"
        @remove="removeItem(index)"
      />
    </div>
  </div>
</template>

<script setup>
import { ref } from "vue";
import FormInputField from "../../../components/shared/forms/FormInputField.vue";
import MultipleChoiceItem from "./MultipleChoiceItem.vue";
import SettingsToggle from "../../../components/shared/SettingsToggle.vue";

const items = ref([]); 
const newItem = ref('');
const emit = defineEmits(['update:items']);

function addItem() {
  if (newItem.value.trim()) {
    items.value.push({ text: newItem.value, selected: false });
    newItem.value = '';
  }
}

function emitItems() {
  emit('update:items', items.value);
}

function removeItem(index) {
  items.value.splice(index, 1);
  emitSelected();
}

</script>

<style scoped>
svg {
  color: #f97316;
}
</style>
