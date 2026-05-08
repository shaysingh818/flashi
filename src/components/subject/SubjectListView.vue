<template>
  <div class="subject-list-view" v-for="(item, index) in subjectStore.subjects" :key="index">
    <SubjectListItem
      :id="item.id"
      :title="item.name"
      :description="item.description"
      :dateUpdated="item.update"
    />
  </div>
</template>

<script>
import SubjectListItem from './SubjectListItem.vue'
import { subjectStore } from '../../api/Subject';

export default {
  name: 'SubjectListView',
  components: {
    SubjectListItem
  },
  async mounted() {
    await this.subjectStore.loadSubjects();
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
