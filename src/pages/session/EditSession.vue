<template>
 <div class="flex col-center w-100 g-1">
   <br></br>

    <FormHeader
      header="Edit study session"
      description="Edit attributes for existing study session."
    />
    <br />

    <EditSessionForm
      sessionName="Session Name"
      sessionDescription="Enter a description"
      :submitHandler="submitSubject"
    />
  </div>
</template>


<script>
import EditSessionForm from '../../components/session/EditSessionForm.vue';
import FormHeader from '../../components/shared/forms/FormHeader.vue';

export default {
  name: "EditSession",
  components: { EditSessionForm, FormHeader },
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
