<template>
  <div class="flex items-center justify-center w-full">
    <table>
      <tbody>
        <tr>
          <th>Date Started</th>
          <th>Review Time</th>
          <th>Questions Reviewed</th>
          <th>Score</th>
        </tr>

        <SessionReviewTableItem
            v-for="(item, index) in sessionReviewStore.session_review_metrics"
            :key="index"
            :id="item.id"
            :index="index"
            :reviewTime="item.review_time"
            :dateStarted="item.created"
            :totalReviewCount="item.total_review_count"
            :accuracy="item.accuracy"
        /> 
        
      </tbody>
    </table>
  </div>
</template>

<script setup>
import { onMounted } from 'vue'; 
import { sessionReviewStore } from '../../api';
import Tag from '../shared/Tag.vue';
import SessionReviewTableItem from './SessionReviewTableItem.vue';


const props = defineProps({
  sessionId: {
    type: String,
    required: true,
  }
})

onMounted(async () => {
  await sessionReviewStore.loadSessionReviewMetricsHistory(`${props.sessionId}`);
})

</script>

<style scoped>

table {
  width: 90%;
  background-color: rgb(28 25 23);
  border-radius: 10px;
  border-collapse: collapse; /* 👈 add this */
}

tbody tr {
  border-bottom: 2px solid rgb(41 37 36); 
}

th {
  color: white;
  padding: 10px;
  font-weight: normal; 
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
