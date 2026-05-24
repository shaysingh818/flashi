<template>
  <div
    class="flex centered-all p-4 tag"
    :style="{width: props.width, height: props.height }"
    @dblclick="$emit('remove')"
  >
    <p>{{ title }}</p>
  </div>
</template>

<script setup>

const props = defineProps({
    title: {
      type: String,
      required: true,
    },
    width: {
      type: String,
      default: "auto",
    },
    height: {
      type: String,
      default: "auto",
    },
});

defineEmits(['remove']);
</script>

<style scoped>
.tag {
  border-radius: 5px;
  background-color: #f97316;
  padding: 5px;
  white-space: nowrap;
}

p {
  font-weight: normal;
  font-size: 10px;
  color: white;
}
</style>
