<template>
<div class="p-6"> 
    <div class="flex flex-col" v-for="(item, index) in subjectStore.subjects" :key="index">
      <p v-if="subjectStore.subjects.length === 0">No subjects found.</p>
      <SubjectListItem
        :id="`${item.id}`"
        :title="item.name"
        :description="item.description"
        :dateUpdated="item.updated"
      />
    </div>
</div>
</template>

<script setup>
import SubjectListItem from './SubjectListItem.vue';
import { subjectStore } from '../../api/Subject';
import { onMounted } from 'vue';

onMounted(async () => {
  console.log(subjectStore.subjects);
  await subjectStore.loadSubjects();
});
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
