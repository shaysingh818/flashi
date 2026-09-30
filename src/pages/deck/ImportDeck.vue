<template>
  <div class="flex flex-col">

    <GnBackBar 
        title="Import Deck"
        subtitle="Last updated 04/02/2026"
        textColor="var(--app-color-text)"
        color="var(--app-color-accent)"
        backgroundColor="var(--app-color-surface)"
        borderColor="var(--app-color-surface)"
        @gn-back="goBack">
        <font-awesome-icon icon="fa-solid fa-arrow-left" />  
    </GnBackBar>
      
     <FormHeader
      header="Import a deck"
      description="Decks can be exported by clicking on the 'Export Deck' option on the right side of a deck list item. You can import decks that have been exported by selecting the folder of the exported deck."
     />
     <br />

     <ImportDeckForm :subjectId="subjectId" />
     
  </div>
</template>

<script setup>
import { useRouter, useRoute } from "vue-router";
import FormHeader from '../../components/shared/forms/FormHeader.vue';
import ImportDeckForm from '../../components/deck/ImportDeckForm.vue';

const router = useRouter();
const route = useRoute();
const subjectId = route.params.id;

function goBack() {
  router.go(-1);
}

</script>

