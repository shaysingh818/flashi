<template>
  <div class="flex flex-col gap-4 p-4">

    <GnFormInputField
        @update:model-value="name = $event"
        :model-value="name"
        backgroundColor="var(--app-color-surface)" 
        borderColor="transparent" 
        :disabled="false" 
        textColor="var(--app-color-accent)" 
        label="Enter session name"
        type="text" />

    <GnFormTextField
        @update:model-value="description = $event"
        :model-value="description"
        backgroundColor="var(--app-color-surface)" 
        borderColor="transparent" 
        :disabled="false" 
        placeholder="Write something…" 
        :rows="4"        
        label="Enter deck description"
        textColor="var(--app-color-accent)" />

    <div class="flex flex-row gap-4 items-center justify-center">
        <GnButton :disabled="false" variant="secondary" @gn-click="goBack">
            Discard
        </GnButton>
        <GnButton :disabled="false" variant="primary" @gn-click="submitForm">
            Submit
        </GnButton>
    </div>

  </div>
</template>

<script setup>
import { ref } from "vue";
import { useRouter } from "vue-router";
import { sessionStore } from "../../api";

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
}); 

const router = useRouter(); 
const name = ref(props.sessionName);
const description = ref(props.sessionDescription);


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
