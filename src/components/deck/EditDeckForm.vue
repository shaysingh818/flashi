<template>
  <div class="flex flex-col gap-4 p-4">

    <GnFormInputField 
        @update:model-value="name = $event"
        :model-value="props.deckName"
        backgroundColor="var(--app-color-surface)" 
        borderColor="transparent" 
        :disabled="false" 
        textColor="var(--app-color-accent)" 
        label="Enter deck name"
        type="text" />

    <GnFormTextField
        @update:model-value="description = $event"
        :model-value="props.deckDescription"
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
        @gn-dblclick="removeTag(index)"
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
import { ref } from "vue";
import { useRouter } from "vue-router";
import { deckStore } from "../../api/Deck";
import { message } from '@tauri-apps/plugin-dialog';

const props = defineProps({   
  deckId: {
    type: String,
    required: true,
  },  
  deckName: {
    type: String,
    required: true,
  },
  deckDescription: {
    type: String,
    required: true,
  },
  tags: {
    type: Array,
    required: true,
  },
})

const router = useRouter(); 
const tags = ref(props.tags);
const newTag = ref(''); 
const name = ref(props.deckName);
const description = ref(props.deckDescription);

function goBack() {
  router.go(-1);
}

function addTag() {  
  tags.value.push(newTag.value);
  newTag.value = "";
}

async function submitForm() {  
  const currDeck = await deckStore.viewDeck(props.deckId);
  const editDeckResult = await deckStore.editDeck(
    `${props.deckId}`, 
    name.value, 
    description.value, 
    tags.value, 
    currDeck.reviewed
  );

  if (editDeckResult === "Success") {    
    await message(
      'Updated deck successfully',
      { title: 'Recalli', kind: 'info' }
    );
  } else {    
    await message(
      `There was an error updating this deck ${editDeckResult}`,
      { title: 'Recalli', kind: 'info' }
    );
  }
}
</script>

