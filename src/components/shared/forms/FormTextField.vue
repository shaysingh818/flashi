<template>
    <textarea
      :value="text"
      :placeholder="`${ description }`"
      @input="$emit('update:text', $event.target.value)"
      rows="4"
    ></textarea>
</template>

<script>

export default {
  name: 'FormTextField',
  props: {    
    text: {
      type: String,
      required: true,
    },
    description: {
      type: String,
      required: true,
    },
  },
  emits: ["update:text"],
}

</script>

<style scoped>

textarea {
  width: 100%;
  background-color: rgb(28 25 23);
  border: none;
  border-radius: 5px;
  padding: 5px;
  resize: none;
  color: #F97316; 
}

</style>
