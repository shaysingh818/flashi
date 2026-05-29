<template>
    <textarea
      :style="{ backgroundColor: props.backgroundColor }"
      :value="props.text"
      :placeholder="props.description"
      @input="$emit('update:text', $event.target.value)"
      rows="4"
    ></textarea>
</template>

<script setup>


const props = defineProps({   
    text: {
      type: String,
      required: true,
    },
    description: {
      type: String,
      required: true,
    },
    backgroundColor: {
      type: String,
      default: () => "rgb(28 25 23)"
    }
})

defineEmits(["update:text"]);
</script>

<style scoped>

textarea {
  width: 100%;
  border: none;
  border-radius: 5px;
  padding: 5px;
  resize: none;
  color: #F97316; 
}

</style>
