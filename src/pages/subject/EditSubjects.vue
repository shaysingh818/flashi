<template>
  <div class="container col-center">
    <br></br>
    <FormHeader
      header="Edit subject"
      description="Edit Attributes for subject"
    />
    <br />

    <CreateSubjectForm
      :subjectNameText="`${subjectName}`"
      :subjectDescriptionText="`${subjectDescription}`"
      submitText="Submit"
      submitHandler="editSubject"
    />
  </div>
</template>

<script>
import CreateSubjectForm from "../../components/subject/CreateSubjectForm.vue";
import FormHeader from "../../components/shared/FormHeader.vue";

export default {
  name: "EditSubjects",
  components: { CreateSubjectForm, FormHeader },
  data() {
    return {
      subjectName: "testing",
      subjectDescription: "current description",
    };
  },
  methods: {
    editSubject() {
      console.log("Clicked");
    },
    goBack() {
      this.$router.go(-1);
    },
  },
};
</script>
