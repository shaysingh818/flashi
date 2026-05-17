<template>
  <div class="flex col-center">
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

    <CreateQuestionForm
      :deckId="deckId"
      questionText="Question text"
    />
  </div>
</template>

<script setup>
import { ref } from "vue";
import { useRouter, useRoute } from "vue-router";
import FormHeader from "../../components/shared/forms/FormHeader.vue";
import CreateQuestionForm from "../../components/question/CreateQuestionForm.vue";

const router = useRouter(); 
const route = useRoute(); 
const deckId = route.params.id;

function goBack() {
  router.go(-1);
}

</script>

