<template>
 <div class="flex flex-col g-1">
   <br>
    <FormHeader
      header="Create a study session"
      description="Study sessions allow you to add specific decks. Once decks are
      added, you can start doing review sessions and track your metrics around 
      accuracy for all your questions to test understanding."
    />
    <br />

    <CreateSessionForm />
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
