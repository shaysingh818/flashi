<template>
    <textarea
      :placeholder="`${ description }`"
      rows="4"
    ></textarea>
</template>

<script>

export default {
  name: 'FormTextField',
  props: {
    description: {
      type: String,
      required: true,
    },
  },
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
