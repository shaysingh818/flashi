<template>
  <div class="flex col-center gap-4 w-100">
    <div class="flex w-75" style="height: 40px;">
      <FormInputField :text="`${sessionNameText}`" />
    </div>

    <div class="flex w-75" style="height: 60px;">
      <FormTextField :description="`${sessionDescriptionText}`" />
    </div>

    <div class="flex row-center w-75 gap-4" style="height: 40px;">
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

export default {
  name: "CreateSessionForm",
  components: {
    FormInputField,
    FormTextField,
    PrimaryButton,
    SecondaryButton,
  },
  props: {
    sessionNameText: {
      type: String,
      required: true,
    },
    sessionDescriptionText: {
      type: String,
      required: true,
    },
    submitText: {
      type: String,
      required: true,
    },
    submitHandler: {
      type: Function,
      required: true,
    },
  },
  methods: {
    goBack() {
      this.$router.go(-1);
    },
  },
};
</script>
