<template>
  <div class="flex col-center gap-4 w-75 p-4 h-auto">
    <div class="flex w-100" style="height: 40px">
      <FormInputField v-model:text="name" placeholder="Deck name" />
    </div>

    <div class="flex w-100" style="height: 100px;">
      <FormTextField v-model:text="description" description="Deck description" />
    </div>

    <div class="flex col-center gap-2 w-100">
      <div class="flex row-center gap-2 w-100" style="height: 40px">
        <FormInputField
          placeholder="Add a tag to the deck"
          :text="newTag"
          @update:text="newTag = $event"
        />
        <font-awesome-icon icon="fa-solid fa-plus" @click="addTag" />
      </div>
    </div>

    <div class="flex wrap items-start gap-2 row g-1" style="min-height: 15px;">
      <Tag
        v-for="(item, index) in tags"
        :key="index"
        :title="item"
        height="10px"
      />
    </div>

    <br> </br>
    <div class="flex row-center gap-4" style="height: 35px">
      <SecondaryButton @click="goBack" text="Discard" />
      <PrimaryButton @click="submitForm" text="Submit" />
    </div>
  </div>
</template>

<script setup>
import { deckStore } from "../../api/Deck";
import { ref } from "vue";
import { useRouter } from "vue-router";
import FormInputField from "../../components/shared/forms/FormInputField.vue";
import FormTextField from "../../components/shared/forms/FormTextField.vue";
import PrimaryButton from "../../components/shared/buttons/PrimaryButton.vue";
import SecondaryButton from "../../components/shared/buttons/SecondaryButton.vue";
import Tag from "../shared/Tag.vue";

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
  console.log("Submit deck form call...");
  const insertDeck = await deckStore.createDeck(props.subjectId, name.value, tags.value, description.value);
  console.log(insertDeck);
}
</script>

<style scoped>
svg {
  color: #f97316;
}
</style>
