<template>

  <div class="flex col-center gap-4 w-75 p-4">
    
    <SessionListItem
      title="Anatomy 1: Nervous System"
      description="Notes and studying for study of the human body"
      dateUpdated="04/20/2026"
    />

    <SessionListItem
      title="Microbiology"
      description="Notes and studying for study of the human body"
      dateUpdated="04/20/2026"
    />

    <SessionListItem
      title="Anatomy 1: Muscular System"
      description="Notes and studying for study of the human body"
      dateUpdated="04/20/2026"
    />
    
  </div>
</template>

<script>
import SessionListItem from './SessionListItem.vue';

export default {
  name: 'SessionListView',
  components: {
    SessionListItem
  }
} 
</script>

