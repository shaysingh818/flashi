<template>
  <div class="flex flex-col">

    <FormHeader
      header="General Settings"
      description="Current settings that can be configured for the application. Only setting that exists for now is to change the directory in which you'd like to export decks for later use."
    />

    <div class="flex flex-col p-6 gap-4">

      <GnFileUploadField
          :filePicker="selectFolder"
          label="Change path for were exported decks are stored"
          accept="image/*" 
          backgroundColor="var(--app-color-surface)" 
          borderColor="var(--app-color-accent)" 
          color="var(--app-color-accent)" 
          :disabled="false" 
          textColor="var(--app-color-accent)" />

      <div class="flex flex-row items-end gap-2">
          <GnFormInputField
              class="flex-1"
              :model-value="exportPath"
              backgroundColor="var(--app-color-surface)" 
              borderColor="transparent" 
              :disabled="false" 
              placeholder="Current path" 
              label="Current storage path for exported decks"
              textColor="var(--app-color-accent)" />
          <GnIconButton label="Add tag" variant="ghost" @gn-click="addTag">
              <font-awesome-icon icon="fa-solid fa-plus" />
          </GnIconButton>
      </div> 

    </div> 
    
  </div>
</template>


<script setup>
import { ref, onMounted } from "vue";
import {settingsStore} from  '../api'; 
import { message, open } from '@tauri-apps/plugin-dialog';
import SettingsToggle from '../components/shared/SettingsToggle.vue';
import FormHeader from '../components/shared/forms/FormHeader.vue';


const exportPath = ref('');


async function selectFolder() {

  const result = await open({
    directory: true,
    multiple: false,
    recursive: true,
    title: 'Select exported deck folder'
  });

  if(result === null) return [];
  exportPath.value = result;

  return [{
    name: result.split(/[\\/]/).pop() ?? result,
    path: result,
  }]; 
}

onMounted(async () => {
  await settingsStore.loadSettings(); 
  exportPath.value = settingsStore.settings.export_path;
})

</script>
