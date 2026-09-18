<template>
  <div class="flex flex-col">

    <GnBackBar 
        title="Create Deck"
        subtitle="Last updated 04/02/2026"
        textColor="var(--app-color-text)"
        color="var(--app-color-accent)"
        backgroundColor="var(--app-color-surface)"
        borderColor="var(--app-color-surface)"
        @gn-back="goBack">
        <font-awesome-icon icon="fa-solid fa-arrow-left" />  
    </GnBackBar>

    <FormHeader
      header="Edit Deck"
      description="Edit existing deck attributes, change name, tags, description"
    />

    <EditDeckForm
      v-if="deck"
      :deckId="deckId"
      :deckName="deck.name"
      :deckDescription="deck.description"
      :tags="deck.tags"
    />
  </div>
</template>

<script setup>
import { ref } from "vue";
import { onMounted } from "vue";
import { useRoute, useRouter } from "vue-router";
import { deckStore } from "../../api/Deck";
import FormHeader from "../../components/shared/forms/FormHeader.vue";
import EditDeckForm from "../../components/deck/EditDeckForm.vue";

const router = useRouter(); 
const route = useRoute(); 
const deckId = route.params.id.toString();
const deck = ref(null); 

function goBack() {
  router.go(-1); 
}

onMounted(async () => {
  deck.value = await deckStore.viewDeck(deckId);
});
</script>

