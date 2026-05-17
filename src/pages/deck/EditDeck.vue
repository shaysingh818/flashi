<template>
  <div class="flex col-center w-100">
    <br />

    <FormHeader
      header="Edit Deck"
      description="Edit existing deck attributes, change name, tags, description"
    />

    <EditDeckForm
      v-if="deck"
      :deckId="deck.id"
      :deckName="deck.name"
      :deckDescription="deck.description"
      :tags="deck.tags"
    />
  </div>
</template>

<script setup>
import { ref } from "vue";
import { onMounted } from "vue";
import { useRoute } from "vue-router";
import { deckStore } from "../../api/Deck";
import FormHeader from "../../components/shared/forms/FormHeader.vue";
import EditDeckForm from "../../components/deck/EditDeckForm.vue";

const route = useRoute(); 
const deckId = route.params.id;
const deck = ref(null); 

onMounted(async () => {
  deck.value = await deckStore.viewDeck(deckId);
});
</script>

