<template>
  <div class="deck-list-view">
    <DeckListItem
      title="Nervous System"
      description="Notes and studying for study of the human body"
      dateUpdated="04/20/2026"
      :tags="['muscular', 'skeletal', 'organ system']"
    />

    <DeckListItem
      title="Chemical Organization"
      description="Notes and studying for study of the human body"
      dateUpdated="04/20/2026"
      :tags="['muscular', 'skeletal', 'organ system']"
    />

    <DeckListItem
      title="Muscular System"
      description="Notes and studying for study of the human body"
      dateUpdated="04/20/2026"
      :tags="['muscular', 'skeletal', 'organ system']"
    />
  </div>
</template>

<script>
import DeckListItem from "./DeckListItem.vue";
export default {
  name: "DeckListView",
  components: {
    DeckListItem,
  },
};
</script>

<style>
.deck-list-view {
  display: flex;
  flex-direction: column;
  gap: 1rem;
  width: 85%;
  padding: 10px;
  justify-content: center;
  align-items: center;
}
</style>
