<template>
 <div class="flex col-center w-100 g-1">
   <br>
    <FormHeader
      header="Create a study session"
      description="Study sessions allow you to add specific decks. Once decks are
      added, you can start doing review sessions and track your metrics around 
      accuracy for all your questions to test understanding."
    />
    <br />

    <CreateSessionForm
      sessionNameText="Session name"
      sessionDescriptionText="Enter description of session"
      submitText="Submit"
      :submitHandler="submitSession"
    />
  </div>
</template>

<script>
import CreateSessionForm from "../../components/session/CreateSessionForm.vue";
import FormHeader from "../../components/shared/forms/FormHeader.vue";

export default {
  name: "CreateSession",
  components: { CreateSessionForm, FormHeader },
  methods: {
    submitSubject() {
      console.log("Clicked");
    },
    goBack() {
      this.$router.go(-1);
    },
  },
};
</script>
