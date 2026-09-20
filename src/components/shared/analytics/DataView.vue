<template>
<div class="flex flex-col rounded-2 card">

  <br /> 
  <div class="flex justify-center">
    <h2> {{ title }} </h2>
  </div>
  
  <div class="flex flex-col"> 
     <QuestionAccuracyItem
        v-for="(item, index) in props.questions"
        :key="index"
        :score="props.entityType === 'correct' ? item.correct : item.incorrect"
        :totalAttempts="item.reviewed"
        :questionText="item.raw"
    />  
  </div>

</div>

</template>

<script setup>
import QuestionAccuracyItem from '../../question/QuestionAccuracyItem.vue';

const props = defineProps({
  title: {
    type: String,
    required: true,
  },       
  entityType: {
    type: String,
    default: () => "correct",
  },    
  questions: {
    type: Array,
    default: () => [],
  },    
});
</script>

<style>

.card {
  background-color: rgb(28, 25, 23);
  height: 350px;
  overflow-y: auto; 
  justify-content: flex-start;
  border-radius: 10px; 
}

h2 {
  color: white;
  padding-bottom: 0;
  font-size: 20px;
  font-weight: bold; 
}

</style>
