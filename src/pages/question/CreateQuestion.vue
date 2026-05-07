<template>
  <div class="flex col-center">
    <br />
    <FormHeader
      header="Create a deck"
      description="A subject is a top-level category that helps you organize
        related study decks in one place. Think of it like a folder
        — create a subject for a course or topic, then build
        multiple decks inside it to break your material into
        focused, manageable sets."
    />
    <br />

    <CreateQuestionForm
      questionText="Question text"
      :submitHandler="submitDeck"
    />
  </div>
</template>

<script>
import FormHeader from "../../components/shared/forms/FormHeader.vue";
import CreateQuestionForm from "../../components/question/CreateQuestionForm.vue";

export default {
  name: "CreateQuestion",
  components: { FormHeader, CreateQuestionForm },
  methods: {
    submitDeck() {
      console.log("Clicked");
    },
    goBack() {
      this.$router.go(-1);
    },
  },
};
</script>

<style scoped></style>
