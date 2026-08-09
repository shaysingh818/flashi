<template>
  <tr @click="">  
    <td class="date-column">{{ props.dateStarted }}</td>
    <td class="date-column">{{ formattedReviewTime }}</td>
    <td class="date-column"> {{ props.totalReviewCount }}</td>    
    <td class="tag-column"><Tag :title="`${props.accuracy}%`" height="10px" /></td>
  </tr>
</template>

<script setup>
import { ref, computed } from 'vue';
import Tag from '../shared/Tag.vue';
import { useRouter } from 'vue-router';
import { onMounted } from 'vue';
import { answerTypeStore, questionStore } from '../../api';

const props = defineProps({
  reviewTime: {
    type: Number,
    required: true,
  },
  dateStarted: {
    type: String,
    required: true,
  },
  totalReviewCount: {
    type: String,
    required: true,
  },
  accuracy: {
    type: String,
    required: true,
  },
});

const router = useRouter(); 
const answerTypeText = ref('');
const emit = defineEmits(["update:delete"]); 
const formattedReviewTime = computed(() => formatDuration(props.reviewTime));

function formatDuration(totalSeconds) {
  const hours = Math.floor(totalSeconds / 3600);
  const minutes = Math.floor((totalSeconds % 3600) / 60);
  const seconds = Math.floor(totalSeconds % 60);

  if (hours > 0) {
    return `${hours}h ${minutes}m`;
  } else if (minutes > 0) {
    return `${minutes}m ${seconds}s`;
  } else {
    return `${seconds}s`;
  }
}

function questionDetail() {
  questionStore.currQuestion = props.index;
  router.push({ path: `/question/view/${props.id}` });
}

async function deleteQuestion() {
  emit("update:delete", props.id); 
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
  width: 15%; 
}

.content-column {
  color: #b5b3b3;
  font-size: 10px; 
  text-align: left;
  width: 65%; 
}


.delete-column {
  color: #b5b3b3;
  font-size: 10px; 
  text-align: center;
  width: 10%; 
}

</style>

