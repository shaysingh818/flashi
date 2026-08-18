<template>

  <div class="flex col-center w-100">

    <br />

    <FormHeader
      header="Study Metrics"
      description="Track your study activity across time. View how long you've been reviewing, how many questions you've tackled, and how consistently you're building your knowledge.
      These metrics are captured across all subjects, decks and questions reviewed."
    />
    
    <div class="flex items-left w-90">
      <h2> Review Time </h2>
    </div>
    
    <div class="grid cols-3 w-90 gap-6 place-center p-6">
      <MetricCard metric="Daily Review Time" :numeric="dailyReviewTime" />
      <MetricCard metric="Weekly Review Time" :numeric="weeklyReviewTime" />
      <MetricCard metric="Monthly Review Time" :numeric="monthlyReviewTime" />
    </div>
    
    <div class="flex items-left w-90">
      <h2> Questions </h2>
    </div>
    
    <div class="grid cols-3 w-90 gap-6 place-center p-6">
      <MetricCard metric="Questions reviewed all time" :numeric="questionReviewedAllTime" />
      <MetricCard metric="Questions reviewed per day" :numeric="dailyQuestionReviewed" />
      <MetricCard metric="Questions reviewed per week" :numeric="weeklyQuestionReviewed" />
    </div>

    
    <div class="flex items-left w-90">
      <h2> Decks </h2>
    </div>
    
    <div class="grid cols-3 w-90 gap-6 place-center p-6">
      <MetricCard metric="Decks reviewed all time" numeric="20" />
      <MetricCard metric="Decks reviewed per day" numeric="20" />
      <MetricCard metric="Decks created per month" numeric="20" />
    </div>
    
    <div class="grid cols-1 cols-responsive w-95 gap-4">
      <LineChart title="Historical review metrics" />
    </div>

    <br />
    
    <div class="grid cols-1 cols-responsive w-95 gap-4">
      <LineChart 
        title="Question correct/incorrect ratio"
        :datasets="[
          { label: 'Correct', data: [40, 55, 60, 70, 59, 48], color: '#34D399'},          
          { label: 'Incorrect', data: [20, 30, 21, 42, 19, 89], color: '#F43F5E'},
        ]"
      />
    </div>
    
    <br />
    
    <div class="grid cols-1 cols-responsive w-95 gap-4">
      <LineChart 
        title="Question type breakdown"
        :datasets="[
          { label: 'Open Ended', data: [40, 55, 60, 70, 59, 48], color: '#F97316'},          
          { label: 'Multiple Choice', data: [20, 30, 21, 42, 19, 89], color: '#FB923C'},
          { label: 'Ordered Sequence', data: [25, 90, 70, 35, 40, 56], color: '#FDBA74'}
        ]"
      />
    </div>

    <br /> 
    <br /> 
  </div>


</template>


<script setup>
import { ref, onMounted }  from "vue"; 
import { analyticsStore } from "../api/Analytics"; 
import MetricCard from '../components/shared/analytics/MetricCard.vue';
import LineChart from '../components/shared/analytics/LineChart.vue';
import DataView from '../components/shared/analytics/DataView.vue';
import FormHeader from '../components/shared/forms/FormHeader.vue';


const dailyReviewTime = ref(0); 
const weeklyReviewTime = ref(0); 
const monthlyReviewTime = ref(0); 

const questionReviewedAllTime = ref(0); 
const dailyQuestionReviewed = ref(0); 
const weeklyQuestionReviewed = ref(0);

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

onMounted(async () => {
  
  [
    dailyReviewTime.value,
    weeklyReviewTime.value,    
    monthlyReviewTime.value,
  ] = await analyticsStore.reviewTimeMetrics(); 

  dailyReviewTime.value = formatDuration(dailyReviewTime.value); 
  weeklyReviewTime.value = formatDuration(weeklyReviewTime.value);
  monthlyReviewTime.value = formatDuration(monthlyReviewTime.value);

  [
    questionReviewedAllTime.value,
    dailyQuestionReviewed.value,    
    weeklyQuestionReviewed.value,
  ] = await analyticsStore.questionReviewTimeMetrics(); 
  
});


</script>

<style scoped>
h2 {
  margin-bottom: 0;
}

</style>