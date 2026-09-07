<template>

  <div class="flex flex-col p-4" v-for="(item, index) in sessionStore.sessions" :key="index">
    <SessionListItem
      :id="`${item.id}`"
      :title="item.name"
      :description="item.description"
      :dateUpdated="item.updated"
    /> 
  </div>
</template>

<script setup>
import { onMounted } from "vue"; 
import { sessionStore } from "../../api";
import SessionListItem from './SessionListItem.vue';


onMounted(async () => {
  await sessionStore.loadSessions();
});
</script>

