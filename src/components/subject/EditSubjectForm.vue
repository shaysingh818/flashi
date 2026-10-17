<template>
  <div class="flex flex-col gap-4 p-4">

    <GnFormInputField 
        @update:model-value="name = $event"
        :model-value="props.subjectName"
        backgroundColor="var(--app-color-surface)" 
        borderColor="transparent" 
        :disabled="false" 
        textColor="var(--app-color-accent)" 
        label="Enter deck name"
        type="text" />

    <GnFormTextField
        @update:model-value="description = $event"
        :model-value="props.subjectDescription"
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
import { ref } from 'vue';
import { useRouter } from 'vue-router';
import { subjectStore } from '../../api/Subject';
import { message } from '@tauri-apps/plugin-dialog';

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
  if (editSubject === "Success") {    
    await message(
      'Updated subject successfully',
      { title: 'Recalli', kind: 'info' }
    );
  } else {    
    await message(
      `There was an error updating this subject ${editSubject}`,
      { title: 'Recalli', kind: 'info' }
    );
  }
}
</script>
