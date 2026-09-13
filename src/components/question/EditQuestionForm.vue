<template>
  <div class="gn-edit-question-overlay">

      <div class="gn-edit-question-box" @click.stop> 
          <h1 class="gn-edit-question-box__title"> Edit Question Text </h1>

          <div class="w-full"> 
              <GnFormTextField
                @update:model-value="newText = $event"
                :model-value="props.questionText"
                backgroundColor="var(--app-color-surface)" 
                borderColor="transparent" 
                :disabled="false" 
                :rows="4"        
                label="Edit question text"
                textColor="var(--app-color-accent)" />
          </div>
              
        <div class="flex flex-row gap-4">
            <GnButton variant="secondary" @gn-click="props.closeFormHandler">Discard</GnButton>
            <GnButton variant="primary" @gn-click="submitQuestionText">Submit</GnButton>
        </div>
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
  console.log("QUESTION ID PROP: ", props.questionId); 
  const currQuestion = await questionStore.viewQuestion(props.questionId);
  const result = await questionStore.editQuestion(
    props.questionId,
    newText.value,
    currQuestion.reviewed,
    currQuestion.correct,
    currQuestion.incorrect
  ); 
  if (result === "Success") {
    props.closeFormHandler(); 
  } else {
    console.error("Error editing question"); 
  }
}

</script>

<style scoped>

.gn-edit-question-overlay {
  position: fixed;
  inset: 0;
  display: flex;
  align-items: center;
  justify-content: center;
  background-color: rgba(0, 0, 0, 0.5);
  z-index: 1000;
  flex-direction: column;
}

.gn-edit-question-box {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 1.0rem;
  width: min(90%, 50rem);
  padding: 1.25rem;
  background-color: var(--app-color-background);
  border-radius: 0.5rem;
  box-sizing: border-box;
}

.gn-edit-question-box__title {
  font-weight: bold;
  color: #ffffff;
  margin: 0;
}
</style>
