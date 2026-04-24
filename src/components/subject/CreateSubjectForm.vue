<template>
  <div class="create-subject-form">
    <div class="subject-input">
      <FormInputField :text="`${subjectNameText}`" />
    </div>

    <div class="subject-text-input">
      <FormTextField :description="`${subjectDescriptionText}`" />
    </div>

    <div class="subject-form-button-section">
      <SecondaryButton :onClickHandler="goBack" text="Discard" />

      <PrimaryButton :onClickHandler="submitHandler" :text="`${submitText}`" />
    </div>
  </div>
</template>

<script>
import FormInputField from "../../components/shared/FormInputField.vue";
import FormTextField from "../../components/shared/FormTextField.vue";
import PrimaryButton from "../../components/shared/PrimaryButton.vue";
import SecondaryButton from "../../components/shared/SecondaryButton.vue";

export default {
  name: "CreateSubjectForm",
  components: {
    FormInputField,
    FormTextField,
    PrimaryButton,
    SecondaryButton,
  },
  props: {
    subjectNameText: {
      type: String,
      required: true,
    },
    subjectDescriptionText: {
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

<style scoped>
.create-subject-form {
  display: flex;
  flex-direction: column;
  padding: 10px;
  justify-content: center;
  align-items: center;
  width: 80%;
  gap: 2rem;
}

.subject-input {
  width: 100%;
  height: 40px;
}

.subject-text-input {
  width: 100%;
}

.subject-form-button-section {
  width: 100%;
  height: 40px;
  display: flex;
  flex-direction: row;
  align-items: center;
  justify-content: center;
  gap: 0.5rem;
  padding: 10px;
}

input {
  width: 100%;
  height: 100%;
}
</style>
