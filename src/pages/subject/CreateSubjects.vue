<template>

  <div class="page-container">


    <div class="create-subject-header">

      <div class="header-text">

        <h1> Create a subject </h1> 

        <p> A subject is a top-level category that helps you organize related study decks in one place. Think of it like a folder — create a subject for a course or topic, then build multiple decks inside it to break your material into focused, manageable sets. </p>
      
      </div>


    </div>

    <br> 

    <CreateSubjectForm
      subjectNameText="Subject name"
      subjectDescriptionText="Enter a description"
      :submitHandler="submitSubject"
    />


  </div>


</template>


<script>
import CreateSubjectForm from '../../components/subject/CreateSubjectForm.vue'

export default {
  name: 'CreateSubjects',
  components: { CreateSubjectForm },
  methods: {
    submitSubject() {
      console.log("Clicked"); 
    },
    goBack() {
      this.$router.go(-1); 
    }
  }
}
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
  width: 80%;
  gap: 2.0rem;
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

.header-text {
  padding: 10px; 
}

input {
  width: 100%;
  height: 100%; 
}


h1 {
  font-size: 25px; 
}

p {
  color: #B5B3B3;
  font-size: 15px; 
}

</style>
