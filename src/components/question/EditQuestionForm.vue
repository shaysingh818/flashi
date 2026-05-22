<template>
  <div
    style="background-color: rgb(28 25 23)"
    class="flex col-center gap-4 p-4 w-75 centered-overlay  rounded-2">

    <h1 style="font-weight: bold;"> Edit Question Text </h1>
    
    <div class="flex w-75" style="height: 250px">
      <FormTextField 
        :text="props.questionText" 
        :description="props.questionText" 
        backgroundColor="rgb(41 37 36)"
        @update:text="newText = $event;"
      />
    </div>
    
    <br />
    <div class="flex row-center gap-4" style="height: 35px">
      <SecondaryButton :onClickHandler="props.closeFormHandler" text="Discard" />
      <PrimaryButton :onClickHandler="submitQuestionText" text="Submit" />
    </div>
  </div>
</template>

<script setup>
import { ref } from "vue";
import { onMounted } from "vue";
import { useRoute, useRouter } from "vue-router";
import { questionStore } from "../../api/Question";
import FormTextField from "../../components/shared/forms/FormTextField.vue";
import PrimaryButton from "../../components/shared/buttons/PrimaryButton.vue";
import SecondaryButton from "../../components/shared/buttons/SecondaryButton.vue";


const props = defineProps({
  questionId: {
    type: String,
    required: true,
  },
  questionText: {
    type: String,
    required: true,
  },
  closeFormHandler: {
    type: Function,
    required: true
  }
});

const router = useRouter(); 
const newText = ref(''); 

function goBack(){
  router.go(-1); 
}


async function submitQuestionText() {
  console.log(newText.value);
  const result = await questionStore.editQuestion(props.questionId, newText.value);
  console.log(result); 
  if (result === "Success") {
    props.closeFormHandler(); 
  }
}

</script>

<style scoped>

.edit-form {
  background-color: black;
}

</style>
