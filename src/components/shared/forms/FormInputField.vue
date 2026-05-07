<template>
  <input
    type="text"
    :value="text"
    id="search-term"
    name="search-item"
    :placeholder="placeholder"
    @input="$emit('update:text', $event.target.value)"
    class="focus:outline-none focus:ring-0 focus:shadow-none"
  />
</template>

<script>
export default {
  name: "FormInputField",
  props: {
    text: {
      type: String,
      required: true,
    },
    placeholder: {
      type: String,
      required: true,
    },
  },
  emits: ["update:text"],
};
</script>

<style scoped>
input {
  border-radius: 3px;
  width: 100%;
  height: 100%;
  background-color: rgb(28 25 23);
  color: #f97316;
  border: none;
  padding-left: 5px;
}

input::placeholder {
  padding-left: 5px;
}
</style>
