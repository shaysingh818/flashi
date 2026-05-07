<template>

  <div class="flex col-center w-75 gap-2">
    
    <div class="flex p-4 w-100">
      <FormTextField description="Type open ended answers" />
    </div>
    
    <div class="flex row-center gap-2 w-100" style="height: 35px;">
      <FormInputField
        placeholder="Add keyword for open ended answer"
        @update:text="newKeyword = $event"
      />
      <font-awesome-icon icon="fa-solid fa-plus" @click="addKeyword" />
    </div>

    <div class="flex row-center align-left wrap gap-2 p-2">
      <Tag
        v-for="(item, index) in keywords"
        :key="index"
        :title="item"
        height="10px"
      />
    </div>
  </div>

</template>

<script>
import FormInputField from "../../../components/shared/forms/FormInputField.vue";
import FormTextField from "../../../components/shared/forms/FormTextField.vue";
import Tag from "../../../components/shared/Tag.vue";

export default {
  name: "CreateOpenEndedForm",
  components: {
    FormInputField,
    FormTextField,
    Tag,
  },
  data() {
    return {
      keywords: [],
      newKeyword: "",
    };
  },
  methods: {
    addKeyword() {
      this.keywords.push(this.newKeyword);
      this.newKeyword = "";
    },
  },
};
</script>

<style scoped>

svg {
  color: #f97316;
}

</style>