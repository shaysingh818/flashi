<template>
  <div class="flex flex-col p-6 gap-4">

    <GnFormInputField 
        v-model="name"
        backgroundColor="var(--app-color-surface)" 
        borderColor="transparent" 
        :disabled="false" 
        placeholder="Session Name" 
        textColor="var(--app-color-accent)" 
        label="Enter  session name"
        type="text" />

    <GnFormTextField
        v-model="description"
        backgroundColor="var(--app-color-surface)" 
        borderColor="transparent" 
        :disabled="false" 
        placeholder="Write something…" 
        :rows="4"        
        label="Enter session description"
        textColor="var(--app-color-accent)" />

    <div class="flex flex-row gap-4 items-center justify-center">
        <GnButton :disabled="false" variant="secondary" @gn-click="goBack">
            Discard
        </GnButton>
        <GnButton :disabled="false" variant="primary" @gn-click="submitSession">
            Submit
        </GnButton>
    </div>

  </div>
</template>

<script setup>
import { ref } from "vue";
import { useRoute, useRouter } from "vue-router";
import { message } from '@tauri-apps/plugin-dialog';
import { sessionStore } from "../../api";

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
