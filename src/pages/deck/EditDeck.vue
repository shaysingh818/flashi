<template>
  <div class="container centered align-center col">
    <br />

    <FormHeader
      header="Edit Deck"
      description="Edit existing deck attributes, change name, tags, description"
    />

    <EditDeckForm
      deckName="Deck name"
      deckDescription="Description for deck"
      :submitHandler="submitDeck"
    />
  </div>
</template>

<script>
import FormHeader from "../../components/shared/FormHeader.vue";
import EditDeckForm from "../../components/deck/EditDeckForm.vue";

export default {
  name: "EditDeck",
  components: { FormHeader, EditDeckForm },
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

<style scoped></style>
