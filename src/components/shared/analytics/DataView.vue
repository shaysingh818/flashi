<template>
<div class="flex col-center rounded-2 p-6 card">

  <br /> 
  <div class="flex centered-all w-100">
    <h2> {{ title }} </h2>
  </div>
  
  <div class="flex col-center">
    <QuestionAccuracyItem /> 
    <QuestionAccuracyItem />
    <QuestionAccuracyItem />
    <QuestionAccuracyItem />
    <QuestionAccuracyItem />
    <QuestionAccuracyItem />
    <QuestionAccuracyItem />
  </div>

</div>

</template>

<script>
import QuestionAccuracyItem from '../../question/QuestionAccuracyItem.vue';

export default {
  name: "DataView",
  components: { QuestionAccuracyItem },
  props: {    
    title: {
      type: String,
      required: true,
    },    
  },
  computed: {
  },
};

</script>

<style>

.card {
  background-color: rgb(28, 25, 23);
  height: 300px;
  overflow-y: auto; 
}

h2 {
  color: white;
  padding-bottom: 0;
  font-size: 20px;
}

</style>