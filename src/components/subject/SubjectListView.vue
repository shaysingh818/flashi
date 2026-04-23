<template>

  <div class="subject-list-view">

    <SubjectListItem
      title="Anatomy"
      description="Notes and studying for study of the human body"
      dateUpdated="04/20/2026"
    />

    <SubjectListItem
      title="Anatomy"
      description="Notes and studying for study of the human body"
      dateUpdated="04/20/2026"
    />

    <SubjectListItem
      title="Anatomy"
      description="Notes and studying for study of the human body"
      dateUpdated="04/20/2026"
    />

  </div>

</template>

<script>
import SubjectListItem from './SubjectListItem.vue'
export default {
  name: 'SubjectListView',
  components: {
    SubjectListItem
  }
} 
</script>

<style>

.subject-list-view {
  display: flex;
  flex-direction: column; 
  gap: 1.0rem;
  width: 85%;
  padding: 10px;
  justify-content: center;
  align-items: center; 
}

</style>
