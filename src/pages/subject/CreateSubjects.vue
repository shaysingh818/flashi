<template>
 <div class="flex flex-col">

     <GnBackBar 
         title="Create Subject"
         subtitle="Last updated 04/02/2026"
         textColor="var(--app-color-text)"
         color="var(--app-color-accent)"
         backgroundColor="var(--app-color-surface)"
         borderColor="var(--app-color-surface)"
         @gn-back="goBack">
         <font-awesome-icon icon="fa-solid fa-arrow-left" />  
     </GnBackBar>
     
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
import FormHeader from "../../components/shared/forms/FormHeader.vue";
import { GnBackBar } from "@graphyne/web-components";

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
