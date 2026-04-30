<template>
  <div class="container col-center g-1 w-75 p-10">
    <div class="container">
      <FormTextField :description="`${questionText}`" />
    </div>

    <div class="container centered">
      <DropDownFormField
        title="choose option"
        @update:title="selectedOption = $event"
        :options="[
          'open ended',
          'multiple choice',
          'ordered sequence',
          'matching answer',
        ]"
      />
    </div>

    <CreateMultipleChoiceForm v-if="selectedOption == 'multiple choice'" />
    <CreateOpenEndedForm v-if="selectedOption == 'open ended'" />
    <CreateOrderedSequenceForm v-if="selectedOption == 'ordered sequence'" />

    
    <br />
    <div class="container row-center g-1" style="height: 35px">
      <SecondaryButton :onClickHandler="goBack" text="Discard" />
      <PrimaryButton :onClickHandler="submitHandler" text="Submit" />
    </div>
  </div>
</template>

<script>
import FormInputField from "../../components/shared/FormInputField.vue";
import FormTextField from "../../components/shared/FormTextField.vue";
import PrimaryButton from "../../components/shared/PrimaryButton.vue";
import SecondaryButton from "../../components/shared/SecondaryButton.vue";
import Tag from "../shared/Tag.vue";
import DropDownFormField from "../shared/DropDownFormField.vue";
import CreateMultipleChoiceForm from "../../pages/answer/multiple_choice/CreateMultipleChoiceForm.vue";
import CreateOpenEndedForm from "../../pages/answer/open_ended/CreateOpenEndedForm.vue";
import CreateOrderedSequenceForm from "../../pages/answer/ordered_sequence/CreateOrderedSequenceForm.vue";

export default {
  name: "CreateQuestionForm",
  components: {
    FormInputField,
    FormTextField,
    PrimaryButton,
    SecondaryButton,
    DropDownFormField,
    CreateMultipleChoiceForm,
    CreateOpenEndedForm,
    CreateOrderedSequenceForm,
    Tag
  },
  props: {
    questionText: {
      type: String,
      required: true,
    },
    submitHandler: {
      type: Function,
      required: true,
    },
  },
  data() {
    return {
      tags: [],
      selectedOption: "",
    };
  },
  methods: {
    goBack() {
      this.$router.go(-1);
    },
    submitHandler() {
      this.tags.push(this.newTag);
      this.newTag = "";
    },
  },
};
</script>
