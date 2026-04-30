<template>

  <div class="container col-center w-75 g-1">
    
    <div class="container p-5">
      <FormTextField description="Type open ended answers" />
    </div>
    
    <div class="container row-center g-1" style="height: 35px;">
      <FormInputField
        placeholder="Add keyword for open ended answer"
        @update:text="newKeyword = $event"
      />
      <font-awesome-icon icon="fa-solid fa-plus" @click="addKeyword" />
    </div>
    
    <div class="container row-center align-left wrap g-1">
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
import FormInputField from "../../../components/shared/FormInputField.vue";
import FormTextField from "../../../components/shared/FormTextField.vue";
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