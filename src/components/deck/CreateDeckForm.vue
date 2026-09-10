<template>
  <div class="flex flex-col p-6 gap-4">
    <GnFormInputField 
        v-model="name"
        backgroundColor="var(--app-color-surface)" 
        borderColor="transparent" 
        :disabled="false" 
        placeholder="Deck Name" 
        textColor="var(--app-color-accent)" 
        label="Enter deck name"
        type="text" />

    <GnFormTextField
        v-model="description"
        backgroundColor="var(--app-color-surface)" 
        borderColor="transparent" 
        :disabled="false" 
        placeholder="Write something…" 
        :rows="4"        
        label="Enter deck description"
        textColor="var(--app-color-accent)" />

    <div class="flex flex-row items-end gap-2">
        <GnFormInputField
            class="flex-1"
            v-model="newTag"
            backgroundColor="var(--app-color-surface)" 
            borderColor="transparent" 
            :disabled="false" 
            placeholder="Add Tag…" 
            label="Add tag to deck"
            textColor="var(--app-color-accent)" />
        <GnIconButton label="Add tag" variant="primary" @gn-click="addTag">
            <font-awesome-icon icon="fa-solid fa-plus" />
        </GnIconButton>
    </div> 

    <div class="flex flex-row flex-wrap justify-center gap-2 p-4" style="min-height: 15px;">
      <GnTag
        v-for="(item, index) in tags"
        :key="index"
        :label="item"
        height="10px"
      />
    </div>

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
import { message } from '@tauri-apps/plugin-dialog';
import { useRouter } from "vue-router";
import {
  GnFormInputField,
  GnFormTextField,
  GnButton,
  GnBackBar,
  GnTag
} from '@graphyne/web-components';

const props = defineProps({
  subjectId: {
    type: String,
    required: true
  },
});

const router = useRouter(); 
const tags = ref([]);
const name = ref('');
const newTag = ref('');
const description = ref('');

function goBack() {
  router.go(-1);
}

function addTag() {
  tags.value.push(newTag.value);
  newTag.value = "";
}

async function submitForm() {
  const insertDeck = await deckStore.createDeck(props.subjectId, name.value, tags.value, description.value);
  if (insertDeck === "Success") {    
    await message(
      'Created deck, you can now add questions under this deck',
      { title: 'Recalli', kind: 'info' }
    );
  } else {    
    await message(
      'Could not create deck, please add all required fields',
      { title: 'Recalli', kind: 'info' }
    );
  }
}
</script>