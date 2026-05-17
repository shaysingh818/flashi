<template>
  <tr @click="questionDetail">
    <td class="tag-column"><Tag :title="props.questionType" height="10px" /></td>
    <td class="date-column">{{ props.dateUpdated }}</td>
    <td class="content-column">{{ props.content }}</td>
  </tr>
</template>

<script setup>
import { useRouter } from 'vue-router';

const props = defineProps({
  id: {
    type: String,
    required: true,
  },
  questionType: {
    type: String,
    required: true,
  },
  dateUpdated: {
    type: String,
    required: true,
  },
  content: {
    type: String,
    required: true,
  },
});

const router = useRouter(); 

function questionDetail() {
  router.push({ path: `/question/view/${props.id}` });
}

</script>

<style scoped>

tbody tr {
  border-bottom: 2px solid rgb(41 37 36); 
}

td {
  padding: 10px;
}

.tag-column {
  width: 10%; 
}

.date-column {
  color: #F97316;
  font-size: 10px; 
  text-align: center;
  width: 25%; 
}

.content-column {
  color: #b5b3b3;
  font-size: 10px; 
  text-align: left;
  width: 65%; 
}

</style>

