<template>
  <div class="flex col-center gap-4 w-85 p-2">
    <OrderedSequencePreviewItem
      v-if="props.visible === true"
      v-for="(item, index) in props.items"
      :key="index"
      :index="index+1"
      :text="item"
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

console.log(props.items); 
</script>

<style scoped>

</style>
