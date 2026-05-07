<template>
  <label for="item">
    <input type="checkbox" :id="text" :name="text" value="yes" />
    {{ text }}
    <font-awesome-icon icon="fa-solid fa-x" @click="addKeyword" />
  </label>
</template>

<script>
import FormInputField from "../../../components/shared/forms/FormInputField.vue";

export default {
  name: "MultipleChoiceItem",
  components: {
    FormInputField,
  },
  props: {
    text: {
      type: String,
      required: true,
    },
  },
  data() {
    return {
      tags: [],
      selectedOption: "",
    };
  },
  methods: {},
};
</script>

<style scoped>
label {
  display: flex;
  padding: 10px;
  color: #b5b3b3;
  align-items: center;
  justify-content: left;
  width: 100%;
  gap: 1rem;
  font-size: 12px;
}

input[type="checkbox"] {
  appearance: none;
  width: 20px;
  height: 20px;
  background-color: rgb(28 25 23);
  border-radius: 2px;
}

input[type="checkbox"]:checked {
  background-color: #f97316;
  border-color: #007bff;
}

svg {
  color: #f97316;
}
</style>
