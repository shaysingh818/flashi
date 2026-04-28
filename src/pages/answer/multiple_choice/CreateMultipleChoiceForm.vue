<template>
  <div class="container col-center g-1">
    <div class="container row-center g-1 w-75" style="height: 40px">
      <FormInputField
        placeholder="Add multiple choice answer"
        @update:text="newItem = $event"
      />
      <font-awesome-icon icon="fa-solid fa-plus" @click="addItem" />
    </div>

    <div class="container col-center p-10 w-75">
      <MultipleChoiceItem
        v-for="(item, index) in items"
        :key="index"
        :text="item"
      />
    </div>
  </div>
</template>

<script>
import FormInputField from "../../../components/shared/FormInputField.vue";
import MultipleChoiceItem from "./MultipleChoiceItem.vue";

export default {
  name: "CreateMultipleChoiceForm",
  components: {
    FormInputField,
    MultipleChoiceItem,
  },
  data() {
    return {
      items: [],
      newItem: "",
    };
  },
  methods: {
    addItem() {
      this.items.push(this.newItem);
      this.newItem = "";
    },
  },
};
</script>

<style scoped>
svg {
  color: #f97316;
}
</style>
