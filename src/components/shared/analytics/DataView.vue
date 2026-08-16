<template>
<div class="flex col-center rounded-2 card">

  <br /> 
  <div class="flex centered-all w-100">
    <h2> {{ title }} </h2>
  </div>
  
  <div class="flex col-center w-100"> 
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
  height: 300px;
  overflow-y: auto; 
  justify-content: flex-start;
}

h2 {
  color: white;
  padding-bottom: 0;
  font-size: 20px;
}

</style>