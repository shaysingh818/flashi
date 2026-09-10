<template>
  <div class="flex flex-col">
    
    <GnBackBar 
        title="Create Question"
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
      header="Create a question"
      description="Questions belong to decks and come in three types: Open Ended for free-text answers (with optional keywords to match against responses), Sequence for ordered steps or processes, and Multiple Choice for single or multi-select answers."
    />
    <br />

    <CreateQuestionForm
      :deckId="deckId"
      questionText="Question text"
    />
  </div>
</template>

<script setup>
import { ref } from "vue";
import { useRouter, useRoute } from "vue-router";
import { BackBar } from "../../components/shared";
import FormHeader from "../../components/shared/forms/FormHeader.vue";
import CreateQuestionForm from "../../components/question/CreateQuestionForm.vue";

const router = useRouter(); 
const route = useRoute(); 
const deckId = route.params.id;

function goBack() {
  router.go(-1);
}
</script>

