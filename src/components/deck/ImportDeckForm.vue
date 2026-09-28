<template>
  <div class="flex flex-col p-6 gap-4">

    <GnFileUploadField
        :filePicker="selectFolder"
        label="Upload folder of exported deck"
        accept="image/*" 
        backgroundColor="var(--app-color-surface)" 
        borderColor="var(--app-color-accent)" 
        color="var(--app-color-accent)" 
        :disabled="false" 
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
import { deckStore } from "../../api/Deck";
import { ref } from "vue";
import { message, open } from '@tauri-apps/plugin-dialog';
import { useRouter } from "vue-router";
import {
  GnFormInputField,
  GnFormTextField,
  GnButton,
  GnTag
} from '@graphyne/web-components';

const props = defineProps({
  subjectId: {
    type: String,
    required: true
  },
});

const router = useRouter(); 
const name = ref('');
const folderPath = ref('');

function goBack() {
  router.go(-1);
}

async function selectFolder() {

  const result = await open({
    directory: true,
    multiple: true,
    recursive: true,
    title: 'Select exported deck folder'
  });

  if(result === null) return [];
  folderPath.value = result;

  return [{
    name: result.split(/[\\/]/).pop() ?? result,
    path: result,
  }]; 
}

async function submitForm() {
  const insertDeck = await deckStore.importDeck(props.subjectId, folderPath.value);
  if (insertDeck === "Success") {    
    await message(
      'Successfully imported deck',
      { title: 'Recalli', kind: 'info' }
    );
  } else {    
    await message(
      'Could not import deck',
      { title: 'Recalli', kind: 'info' }
    );
  }
}
</script>
