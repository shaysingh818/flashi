<template>
  <div class="flex row-center gap-4" style="height: 50px;">
    <Tag :title="`${correct}/${totalAttempts}`" height="10px" /> 
      <p> {{ questionText }}</p>
  </div>
</template>

<script>
import Tag from '../shared/Tag.vue';

export default {
  name: "QuestionAccuracyItem",
  components: {Tag},
  props: {    
    correct: {
      type: String,
      default: () => "10",
    },    
    totalAttempts: {
      type: String,
      default: () => "100",
    },    
    questionText: {
      type: String,
      default: () => "Random example question because I can testing, with long response and extra items for flex testing.",
    },
  },
  methods: {
    questionDetail() {
      this.$router.push({ path: "/question/view" });
    }
  }
};
</script>

<style scoped>

p {

  color: #b5b3b3;
  font-size: 10px; 
  width: 65%; 
}
</style>