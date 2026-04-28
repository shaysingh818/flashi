<template>
 <div class="container col-center">
   <br></br>

    <FormHeader
      header="Create a subject"
      description="A subject is a top-level category that helps you organize
            related study decks in one place. Think of it like a folder
            — create a subject for a course or topic, then build
            multiple decks inside it to break your material into
            focused, manageable sets."
    />
    <br />

    <CreateSubjectForm
      subjectNameText="Subject name"
      subjectDescriptionText="Enter a description"
      submitText="Submit"
      :submitHandler="submitSubject"
    />
  </div>
</template>

<script>
import CreateSubjectForm from "../../components/subject/CreateSubjectForm.vue";
import FormHeader from "../../components/shared/FormHeader.vue";

export default {
  name: "CreateSubjects",
  components: { CreateSubjectForm, FormHeader },
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

<style scoped>
.create-subject-header {
  width: 85%;
  border-radius: 10px;
  background-color: rgb(28 25 23);
}

.create-subject-form {
  display: flex;
  flex-direction: column;
  padding: 10px;
  justify-content: center;
  align-items: center;
  width: 85%;
  gap: 2rem;
}

.subject-input {
  width: 100%;
  height: 30px;
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
