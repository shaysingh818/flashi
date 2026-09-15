<template>
  <div class="flex flex-col gap-4">

    <div class="p-6">
      <SettingsToggle text="Allow random generation of multiple choice answers. This will add multiple choice items that come from other answers to other questions in the deck." />
    </div>
    

    <div class="flex flex-row items-end gap-2">
        <GnFormInputField
            class="flex-1"
            v-model="newItem"
            backgroundColor="var(--app-color-surface)" 
            borderColor="transparent" 
            :disabled="false" 
            placeholder="Add multiple choice item" 
            label="Add multiple choice item"
            textColor="var(--app-color-accent)" />
        <GnIconButton label="Add tag" variant="ghost" size="sm" @gn-click="addItem">
            <font-awesome-icon icon="fa-solid fa-plus" />
        </GnIconButton>
    </div> 

    <div class="flex flex-col p-2">
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

