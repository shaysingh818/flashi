<template>

  <div class="flex col-center w-100">

    <br />

    <FormHeader
      header="Study Metrics"
      description="Track your study activity across time. View how long you've been reviewing, how many questions you've tackled, and how consistently you're building your knowledge.
      These metrics are captured across all subjects, decks and questions reviewed."
    />
    
    <div class="flex items-left w-90">
      <h2> General </h2>
    </div>
    
    <div class="grid cols-3 w-90 gap-6 place-center p-6">
      <MetricCard metric="Subject Count" :numeric="subjectCount" />
      <MetricCard metric="Deck Count" :numeric="deckCount" />
      <MetricCard metric="Session Count" :numeric="sessionCount" />
    </div>
    
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
 
    <div class="grid cols-1 cols-responsive w-95 gap-4">
      <LineChart 
            title="Historical review metrics"
            :labels="historicalDates"
            :datasets="historicalAccuracy" />
    </div>

    <br />
    
    <div class="grid cols-1 cols-responsive w-95 gap-4">
      <LineChart 
            title="Question correct/incorrect ratio"
            :labels="historicalDates"
            :datasets="historicalCorrectIncorrect" />
    </div>
    
    <br />
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

const subjectCount = ref(0); 
const deckCount = ref(0); 
const sessionCount = ref(0);

const historicalDates = ref([]); 
const historicalCorrectIncorrect = ref([]);
const historicalAccuracy = ref([]);

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
    subjectCount.value,
    deckCount.value,    
    sessionCount.value,
  ] = await analyticsStore.generalMetrics(); 

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
 
  dailyQuestionReviewed.value = Math.round(dailyQuestionReviewed.value * 100) / 100;  
  weeklyQuestionReviewed.value = Math.round(weeklyQuestionReviewed.value * 100) / 100;

  
  const historicalMetrics = await analyticsStore.historicalMetrics()
  historicalDates.value = historicalMetrics.map(([date]) => date);  

  const historicalCorrect = historicalMetrics.map(([, correct]) => correct);  
  const historicalIncorrect = historicalMetrics.map(([,,incorrect]) => incorrect);  
  const historicalAccuracyValues = historicalMetrics.map(([,,,accuracy]) => accuracy);

  historicalCorrectIncorrect.value = [    
    { label: 'Correct', data: historicalCorrect, color: '#34D399'},          
    { label: 'Incorrect', data: historicalIncorrect, color: '#F43F5E'},
  ]
  
  historicalAccuracy.value = [    
    { label: 'Accuracy', data: historicalAccuracyValues, color: '#F97316'},          
  ]
  
});


</script>

<style scoped>
h2 {
  margin-bottom: 0;
}

</style>