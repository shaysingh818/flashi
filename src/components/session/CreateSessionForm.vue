<template>
  <div class="flex col-center gap-4 w-100">
    <div class="flex w-75" style="height: 40px;">
      <FormInputField 
        placeholder="Name your session" 
        @update:text="name = $event"
      />
    </div>

    <div class="flex w-75" style="height: 60px;">
      <FormTextField 
        description="Enter description for session"
        @update:text="description = $event"
      />
    </div>

    <div class="flex row-center w-75 gap-4" style="height: 40px;">
      <SecondaryButton @click="goBack" text="Discard" />
      <PrimaryButton @click="submitSession" text="Submit" />
    </div>
  </div>
</template>

<script setup>
import { ref } from "vue";
import { useRoute, useRouter } from "vue-router";
import { message } from '@tauri-apps/plugin-dialog';
import { sessionStore } from "../../api";
import FormInputField from "../../components/shared/forms/FormInputField.vue";
import FormTextField from "../../components/shared/forms/FormTextField.vue";
import PrimaryButton from "../../components/shared/buttons/PrimaryButton.vue";
import SecondaryButton from "../../components/shared/buttons/SecondaryButton.vue";

const router = useRouter(); 
const name = ref(''); 
const description = ref('');

function goBack() {
  router.go(-1);
}

async function submitSession() {

  if(name.value != '' && description.value != '') {
    const insertResult = await sessionStore.createSession(
      name.value,
      description.value
    );
    if (insertResult === "Success") { 
      await message(
        'Created session, you can now add decks and start reviewing questions under this session',
        { title: 'Recalli', kind: 'info' }
      );
    }
  } else {    
    await message(
      'Could not create session, name and description field required',
      { title: 'Recalli', kind: 'info' }
    );
  }

}

</script>
