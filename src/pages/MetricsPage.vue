<template>

  <div class="flex flex-col">

    <FormHeader
      header="Study Metrics"
      description="Track your study activity across time. View how long you've been reviewing, how many questions you've tackled, and how consistently you're building your knowledge.
      These metrics are captured across all subjects, decks and questions reviewed."
    />
    
    <div class="flex items-left p-4">
      <h2> General </h2>
    </div>
    
    <div class="grid grid-cols-3 gap-4 place-center p-4">
      <GnMetricCard metric="Subject Count" :numeric="subjectCount" />
      <GnMetricCard metric="Deck Count" :numeric="deckCount" />
      <GnMetricCard metric="Session Count" :numeric="sessionCount" />
    </div>
    
    <div class="flex items-left p-4">
      <h2> Review Time </h2>
    </div>
    
    <div class="grid grid-cols-3 gap-4 place-center p-4">
      <GnMetricCard metric="Daily Review Time" :numeric="dailyReviewTime" />
      <GnMetricCard metric="Weekly Review Time" :numeric="weeklyReviewTime" />
      <GnMetricCard metric="Monthly Review Time" :numeric="monthlyReviewTime" />
    </div>
    
    <div class="flex items-left p-4">
      <h2> Questions </h2>
    </div>
    
    <div class="grid grid-cols-3 gap-4 place-center p-4">
      <GnMetricCard metric="Questions reviewed all time" :numeric="questionReviewedAllTime" />
      <GnMetricCard metric="Questions reviewed per day" :numeric="dailyQuestionReviewed" />
      <GnMetricCard metric="Questions reviewed per week" :numeric="weeklyQuestionReviewed" />
    </div>
 
    <div class="grid grid-cols-1 gap-4 p-4">
      <GnLineChart
        height="22rem"
        title="Historical review metrics"
        :labels="historicalDates"
        :datasets="historicalAccuracy"
        backgroundColor="var(--app-color-surface)"
        borderColor="var(--app-color-surface)" 
        gridColor="gray"
        textColor="#ffffff"/>
    </div>
 
    <div class="grid grid-cols-1 gap-4 p-4">

      <GnLineChart
        height="22rem"
        title="Question correct/incorrect ratio"
        :labels="historicalDates"
        :datasets="historicalCorrectIncorrect"
        backgroundColor="var(--app-color-surface)"
        borderColor="var(--app-color-surface)" 
        gridColor="gray"
        textColor="#ffffff"/>

    </div>
    
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
  font-weight: bold; 
}

</style>
