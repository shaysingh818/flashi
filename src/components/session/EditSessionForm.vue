<template>
  <div class="flex col-center gap-4 w-100">
    <div class="flex w-75" style="height: 40px;">
      <FormInputField v-model:text="name" :placeholder="`${sessionName}`" />
    </div>

    <div class="flex w-75" style="height: 40px;">
      <FormTextField v-model:text="description" :description="`${sessionDescription}`" />
    </div>

    <div class="flex row-center w-75 gap-4" style="height: 40px;">
      <SecondaryButton :onClickHandler="goBack" text="Discard" />
      <PrimaryButton :onClickHandler="submitForm" text="Save" />
    </div>
  </div>
</template>

<script setup>
import { ref } from "vue";
import { useRouter } from "vue-router";
import { sessionStore } from "../../api";
import FormInputField from "../../components/shared/forms/FormInputField.vue";
import FormTextField from "../../components/shared/forms/FormTextField.vue";
import PrimaryButton from "../../components/shared/buttons/PrimaryButton.vue";
import SecondaryButton from "../../components/shared/buttons/SecondaryButton.vue";

const props = defineProps({ 
    sessionId: {
      type: String,
      required: true,
    },
    sessionName: {
      type: String,
      required: true,
    },
    sessionDescription: {
      type: String,
      required: true,
    },
})


const router = useRouter(); 
const name = ref(props.deckName);
const description = ref(props.deckDescription);


function goBack() {
  router.go(-1); 
}

async function submitForm() {
  const editSession = await sessionStore.editSession(
    `${props.sessionId}`, 
    name.value, 
    description.value
  );
  console.log(editSession);
}
</script>
