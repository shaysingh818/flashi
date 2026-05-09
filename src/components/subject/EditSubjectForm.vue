<template>
  <div class="flex col-center gap-4 w-100">
    <div class="flex w-75" style="height: 40px;">
      <FormInputField v-model:text="name" :placeholder="subjectName"/>
    </div>

    <div class="flex w-75" style="height: 100px;">
      <FormTextField v-model:text="description" :description="subjectDescription" />
    </div>

    <div class="flex row-center w-75 gap-4" style="height: 40px;">
      <SecondaryButton :onClickHandler="goBack" text="Discard" />
      <PrimaryButton :onClickHandler="submitForm" text="Save" />
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
  subjectId: {
    type: String,
    required: true
  },
  subjectName: {
    type: String,
    required: true
  },
  subjectDescription: {
    type: String,
    required: true
  },
});

const router = useRouter();
const name = ref(props.subjectName);
const description = ref(props.subjectDescription);

function goBack() {
  router.go(-1);
}

async function submitForm() {
  const editSubject = await subjectStore.editSubject(`${props.subjectId}`, name.value, description.value);
  console.log(editSubject); 
}
</script>

<style scoped>

input {
  width: 100%;
  height: 100%;
}
</style>