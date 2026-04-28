<template>
  <div class="container centered align-center col">
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
     
     <CreateDeckForm
       deckNameText="Deck name"
       deckDescriptionText="Description for deck"
       deckTagText="Add tags to deck"
       submitText="Submit"
       :submitHandler="submitDeck"
     />
  </div>
</template>

<script>
import FormHeader from '../../components/shared/FormHeader.vue';
import CreateDeckForm from '../../components/deck/CreateDeckForm.vue';

export default {
  name: "CreateDeck",
  components: { FormHeader, CreateDeckForm },
  methods: {
    submitDeck() {
      console.log("Clicked");
    },
    goBack() {
      this.$router.go(-1);
    },
  },
};

</script>

<style scoped>


</style>
