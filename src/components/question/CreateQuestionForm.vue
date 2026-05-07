<template>
  <div class="flex col-center gap-4 w-100">
    <div class="flex w-75" style="height: 150px;">
      <FormTextField :description="`${questionText}`" />
    </div>

    <div class="flex centered-all w-75">
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
    <div class="flex row-center gap-4" style="height: 35px">
      <SecondaryButton :onClickHandler="goBack" text="Discard" />
      <PrimaryButton :onClickHandler="submitHandler" text="Submit" />
    </div>
  </div>
</template>

<script>
import FormInputField from "../../components/shared/forms/FormInputField.vue";
import FormTextField from "../../components/shared/forms/FormTextField.vue";
import PrimaryButton from "../../components/shared/buttons/PrimaryButton.vue";
import SecondaryButton from "../../components/shared/buttons/SecondaryButton.vue";
import Tag from "../shared/Tag.vue";
import DropDownFormField from "../shared/forms/DropDownFormField.vue";
import CreateMultipleChoiceForm from "../answer/multiple_choice/CreateMultipleChoiceForm.vue";
import CreateOpenEndedForm from "../answer/open_ended/CreateOpenEndedForm.vue";
import CreateOrderedSequenceForm from "../answer/ordered_sequence/CreateOrderedSequenceForm.vue";

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
