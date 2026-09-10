<template>
  <div class="flex flex-col">
    <div class="grid grid-cols-3 gap-6 place-center p-4">
      <GnMetricCard metric="Average Accuracy" :numeric="avgAccuracy" />
      <GnMetricCard metric="Total Questions Reviewed" :numeric="totalQuestionsReviewed" />
      <GnMetricCard metric="Total Review Time" :numeric="totalReviewTime" />
    </div>

    <div class="grid grid-cols-2 gap-4 p-4">
        <DoughnutChart 
            title="Question accuracy ratio"
            :values="[totalCorrect, totalIncorrect]" /> 
        
      <PieChart 
            title="Question type breakdown"
            :labels="questionBreakdownLabels"
            :values="questionBreakdownCounts" />
    </div>

    <div class="grid grid-cols-1 cols-responsive gap-4 p-4">
      <LineChart 
            title="Historical review metrics" 
            :labels="accuracyDates"
            :datasets="accuracyHistory" />
    </div>

    <div class="grid grid-cols-2 cols-responsive gap-4 p-4">

        <DataView 
            title="Most incorrect questions"
            entityType="incorrect"
            :questions="incorrectQuestions" />
        
      <DataView 
            title="Most correct question"
            entityType="correct"
            :questions="correctQuestions" />
            
    </div>

    <br />
    <br />
  </div>
</template>

<script setup>
import { ref, onMounted } from "vue"; 
import { sessionStore } from "../../api/Session";

import {
  GnMetricCard,
  GnDoughnutChart,
  GnPieChart
} from "@graphyne/web-components";
import DoughnutChart from "../../components/shared/analytics/DoughnutChart.vue";
import PieChart from "../../components/shared/analytics/PieChart.vue";
import BarChart from "../../components/shared/analytics/BarChart.vue";
import LineChart from "../../components/shared/analytics/LineChart.vue";
import DataView from "../../components/shared/analytics/DataView.vue";

const props = defineProps({
  sessionId: {
    type: String,
    required: true,
  },
});

const avgAccuracy = ref(0.0); 
const totalQuestionsReviewed = ref(0); 
const totalReviewTime = ref(0); 
const totalCorrect = ref(0); 
const totalIncorrect = ref(0); 
const questionBreakdownLabels = ref([]); 
const questionBreakdownCounts = ref([]);
const incorrectQuestions = ref([]); 
const correctQuestions = ref([]);
const accuracyDates = ref([]); 
const accuracyHistory = ref([]); 

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
    avgAccuracy.value,
    totalQuestionsReviewed.value,    
    totalReviewTime.value,
    totalCorrect.value,
    totalIncorrect.value
  ] = await sessionStore.viewSessionMetrics(props.sessionId); 

  totalReviewTime.value = formatDuration(totalReviewTime.value);
  avgAccuracy.value = Math.round(avgAccuracy.value * 100) / 100;
  
  const questionBreakdown = await sessionStore.viewSessionQuestionBreakdown(props.sessionId)
  questionBreakdownLabels.value = questionBreakdown.map(([label]) => label);   
  questionBreakdownCounts.value = questionBreakdown.map(([, count]) => count); 

  const questionAccuracyHistory = await sessionStore.viewSessionDailyAccuracyHistory(props.sessionId)
  accuracyDates.value = questionAccuracyHistory.map(([date]) => date);
  const accuracyHistoryValues = questionAccuracyHistory.map(([, accuracy]) => accuracy);

  console.log(accuracyHistoryValues)
  accuracyHistory.value = [
    { label: "Accuracy", data: accuracyHistoryValues, color: "#F97316"}
  ];

  
  [
    correctQuestions.value,    
    incorrectQuestions.value
  ] = await sessionStore.viewSessionCorrectIncorrectQuestions(props.sessionId); 

  
});

</script>
