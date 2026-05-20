<template>
</template>

<script setup>

const props = defineProps({  
  title: {
    type: String,
    required: true,
  },
  options: {
    type: Array,
    required: false,
    default: () => [],
  },
})

function submitForm() {
  
}

</script>

