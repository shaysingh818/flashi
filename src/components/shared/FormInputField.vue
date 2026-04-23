<template>
    <input type="text" id="search-term" name="search-item" :placeholder=" text "  class="focus:outline-none focus:ring-0 focus:shadow-none">
</template>

<script>

export default {
  name: 'FormInputField',
  props: {
    text: {
      type: String,
      required: true,
    },
  }
}

</script>

<style scoped>


input {
  border-radius: 3px;
  width: 650px;
  height: 100%; 
  background-color: rgb(28 25 23);
  color: #F97316;
  border: none;
  padding: 5px; 
}

@media (min-width: 400px) {
  input { width: 200px; }
}

@media (min-width: 600px) {
  input { width: 200px; }
}

@media (min-width: 800px) {
  input { width: 400px; }
}

@media (min-width: 1000px) {
  input { width: 650px; }
}

@media (min-width: 1200px) {
  input { width: 800px; }
}



</style>
