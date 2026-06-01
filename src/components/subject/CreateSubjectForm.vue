<template>
  <div class="flex col-center gap-4 w-100">
    <div class="flex w-75" style="height: 40px;">
      <FormInputField v-model:text="name" placeholder="Subject Name" />
    </div>

    <div class="flex w-75" style="height: 100px;">
      <FormTextField v-model:text="description" description="Subject Description" />
    </div>

    <div class="flex row-center w-75 gap-4" style="height: 40px;">
      <SecondaryButton @click="goBack" text="Discard" />
      <PrimaryButton @click="submitForm" :text="`${submitText}`" />
    </div>
  </div>
</template>

<script setup>
import { ref } from 'vue';
import { useRouter } from 'vue-router';
import FormInputField from "../../components/shared/forms/FormInputField.vue";
import FormTextField from "../../components/shared/forms/FormTextField.vue";
import PrimaryButton from "../../components/shared/buttons/PrimaryButton.vue";
import SecondaryButton from "../../components/shared/buttons/SecondaryButton.vue";
import { subjectStore } from '../../api/Subject';

const props = defineProps({
  subjectNameText: {
    type: String,
    required: true
  },
  subjectDescriptionText: {
    type: String,
    required: true
  },
  submitText: {
    type: String,
    required: true
  },
  submitHandler: {
    type: Function,
    required: true
  },
});

const router = useRouter();
const name = ref('');
const description = ref('');

function goBack() {
  router.go(-1);
}

async function submitForm() {
  const insertSubject = await subjectStore.createSubject(name.value, description.value); 
  console.log(insertSubject); 
}
</script>

<style scoped>

input {
  width: 100%;
  height: 100%;
}
</style>
