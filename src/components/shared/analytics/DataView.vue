<template>
<div class="flex col-center rounded-2 p-6" style="background-color: rgb(28, 25, 23);">

  <div class="flex centered-all w-100">
    <h2> Most incorrect questions </h2>
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
  computed: {
  },
};

</script>

<style>

h2 {
  color: white;
  padding-bottom: 0;
  margin-top: 0;
  font-size: 20px;
}
</style>