<template>
  <div
    class="flex col-center gap-4 w-75 p-4"
    style="height: auto"
  >
    <div class="flex w-100" style="height: 40px">
      <FormInputField v-model:text="name" :placeholder="`${deckName}`" />
    </div>

    <div class="flex w-100" style="height: 100px;">
      <FormTextField v-model:text="description" :description="`${deckDescription}`" />
    </div>

    <div class="flex col-center gap-2 w-100">
      <div class="flex row-center gap-2 w-100" style="height: 40px">
        <FormInputField
          placeholder="Update tags for deck"
          :text="newTag"
          @update:text="newTag = $event"
        />
        <font-awesome-icon icon="fa-solid fa-plus" @click="addTag" />
      </div>
    </div>

    <div
      class="flex centered align-left row gap-2"
      style="min-height: 15px; flex-wrap: wrap; align-items: flex-start"
    >
      <Tag
        v-for="(item, index) in tags"
        :key="index"
        :title="item"
        height="10px"
      />
    </div>

    <br> </br>
    <div class="flex row-center gap-4" style="height: 35px">
      <SecondaryButton :onClickHandler="goBack" text="Discard" />
      <PrimaryButton :onClickHandler="submitForm" text="Update" />
    </div>
  </div>
</template>

<script setup>
import { ref } from "vue";
import { useRouter } from "vue-router";
import { deckStore } from "../../api/Deck";
import FormInputField from "../../components/shared/forms/FormInputField.vue";
import FormTextField from "../../components/shared/forms/FormTextField.vue";
import PrimaryButton from "../../components/shared/buttons/PrimaryButton.vue";
import SecondaryButton from "../../components/shared/buttons/SecondaryButton.vue";
import Tag from "../shared/Tag.vue";

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
  const editDeck = await deckStore.editDeck(`${props.deckId}`, name.value, description.value, tags.value);
}
</script>

<style scoped>
svg {
  color: #f97316;
}
</style>
