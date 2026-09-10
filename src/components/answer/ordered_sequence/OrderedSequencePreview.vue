<template>
  <div class="flex flex-col gap-4 p-2">

    <div v-if="props.visible === true" class="flex flex-row gap-2">

      <GnFormInputField
          class="flex-1"
          v-model="newSequence"
          backgroundColor="var(--app-color-surface)" 
          borderColor="transparent" 
          :disabled="false" 
          placeholder="Add item for ordered sequence answer" 
          textColor="var(--app-color-accent)" />
          
        <GnIconButton label="Add tag" variant="primary" @gn-click="addSequence">
            <font-awesome-icon icon="fa-solid fa-plus" />
        </GnIconButton>
 
    </div>

    <OrderedSequencePreviewItem
      v-if="props.visible === true"
      v-for="(item, index) in props.items"
      :key="index"
      :index="index+1"
      :text="item"
      :visible="props.visible"
      @update:text="newItem = $event; newIndex = index; emitUpdateItem()"
      @update:delete="deleteIndex = index; emitDelete()"
    />

    <OrderedSequencePreviewItem
      v-if="props.visible === false"
      v-for="(item, index) in props.items"
      :key="index"
      :index="index+1"
      :visible="props.visible"
      text=""
      @update:text="newItem = $event; newIndex = index; emitSubmittedItem()"
    />
  </div>
</template>

<script setup>
import { ref } from "vue";
import FormInputField from "../../../components/shared/forms/FormInputField.vue";
import OrderedSequencePreviewItem from "./OrderedSequencePreviewItem.vue";

const props = defineProps({
  visible: {
    type: Boolean,
    required: true,
  },
  items: {
    type: Array,
    required: true,
  },
});


const selectedItems = ref([]);
const newItem = ref('');
const newIndex= ref('');
const newSequence = ref('');
const deleteIndex = ref('');
const emit = defineEmits([
  "update:text", 
  "update:index", 
  "update:items", 
  "update:delete", 
  "update:add"
]);


function emitSubmittedItem() {

  const obj = {
    item: newItem.value,
    index: newIndex.value,
  }

  const itemIndex = selectedItems.value.findIndex(item => item.index === obj.index);
  if(itemIndex >= 0) {
    selectedItems.value[itemIndex].item = obj.item;
  } else {
    selectedItems.value.push(obj);
  }

  emit('update:items', selectedItems.value);
}

function emitUpdateItem() {
  emit('update:text', newItem.value);
  emit('update:index', newIndex.value);
  props.items[newIndex.value] = newItem.value; 
  emit('update:items', props.items);
}

function emitDelete() {
  props.items.splice(deleteIndex.value, 1);
  emit('update:delete', props.items);
}

function addSequence() {
  props.items.push(newSequence.value); 
  newSequence.value = '';
  emit('update:add', props.items);
}
</script>

<style scoped>

</style>
