<template>
    <div class="flex flex-col p-4 gap-4">
        <GnFormInputField 
            v-model="name"
            backgroundColor="var(--app-color-surface)" 
            borderColor="transparent" 
            :disabled="false" 
            placeholder="Subject Name" 
            textColor="var(--app-color-accent)" 
            label="Enter subject name"
            type="text" />
    
        <GnFormTextField
            v-model="description"
            backgroundColor="var(--app-color-surface)" 
            borderColor="transparent" 
            :disabled="false" 
            placeholder="Write something…" 
            :rows="4"        
            label="Enter subject description"
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
import { ref } from 'vue';
import { useRouter } from 'vue-router';
import { message } from '@tauri-apps/plugin-dialog';
import { subjectStore } from '../../api/Subject';
import { GnFormInputField, GnFormTextField, GnButton } from '@graphyne/web-components';

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
  if (insertSubject === "Success") {    
    await message(
      'Created subject, you can now add decks under this subject',
      { title: 'Recalli', kind: 'info' }
    );
  } else {    
    await message(
      'Could not create subject, please add all required fields',
      { title: 'Recalli', kind: 'info' }
    );
  }
  
}
</script>