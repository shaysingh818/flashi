<template>
  <div class="search-container">
    <FormInputField :text="`${text}`" />
    <PrimaryButton :onClickHandler="onClick" text="Search" />
  </div>
</template>

<script>
import FormInputField from "./FormInputField.vue";
import PrimaryButton from "../buttons/PrimaryButton.vue";

export default {
  name: "SearchComponent",
  components: {
    FormInputField,
    PrimaryButton,
  },
  props: {
    text: {
      type: String,
      required: true,
    },
    onClick: {
      type: Function,
      required: true,
    },
  },
};
</script>

<style scoped>
.search-container {
  display: flex;
  flex-direction: row;
  justify-content: center;
  align-items: center;
  height: 40px;
  width: 90%;
  gap: 0.5rem;
}
</style>
