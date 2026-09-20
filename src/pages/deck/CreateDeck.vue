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
      
    <br />
     <FormHeader
      header="Create a deck"
      description="A subject is a top-level category that helps you organize
        related study decks in one place. Think of it like a folder
        — create a subject for a course or topic, then build
        multiple decks inside it to break your material into
        focused, manageable sets."
     />
     <br />
     
     <CreateDeckForm :subjectId="subjectId" />
  </div>
</template>

<script setup>
import { useRouter, useRoute } from "vue-router";
import FormHeader from '../../components/shared/forms/FormHeader.vue';
import CreateDeckForm from '../../components/deck/CreateDeckForm.vue';

const router = useRouter();
const route = useRoute();
const subjectId = route.params.id;

function goBack() {
  router.go(-1);
}

</script>
