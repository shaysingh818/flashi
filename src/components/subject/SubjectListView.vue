<template>
<div class="p-6"> 
    <div class="flex flex-col" v-for="(item, index) in props.subjects" :key="index">
      <p v-if="props.subjects.length === 0">No subjects found.</p>
      <SubjectListItem
        :id="`${item.id}`"
        :title="item.name"
        :description="item.description"
        :dateUpdated="item.updated"
        :actionType="props.actionType"
        :sessionId="props.sessionId"
      />
    </div>
</div>
</template>

<script setup>
import SubjectListItem from './SubjectListItem.vue';
import { subjectStore } from '../../api/Subject';
import { onMounted } from 'vue';

const props = defineProps({
  subjects: {
    type: Array,
    default: () => [],
  },
  actionType: {
    type: String,
    default: () => "view",
  }, 
  sessionId: {
    type: String,
    required: false,
  },
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
