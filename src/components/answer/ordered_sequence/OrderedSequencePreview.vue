<template>
  <div class="flex col-center gap-4 w-85 p-2">

    <div v-if="props.visible === true" class="flex row-center gap-2 w-95" style="height: 35px">
      <FormInputField
        placeholder="Add item for ordered sequence answer"
        @update:text="newSequence = $event"
      />
      <font-awesome-icon icon="fa-solid fa-plus" style="color: #f97316" @click="addSequence" />
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
      text=""
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


const newItem = ref('');
const newIndex= ref('');
const newSequence = ref('');
const deleteIndex = ref('');
const emit = defineEmits(["update:text", "update:index", "update:items", "update:delete", "update:add"]);


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
