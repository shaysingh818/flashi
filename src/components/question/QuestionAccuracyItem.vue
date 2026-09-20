<template>
  <div class="flex flex-row gap-4 p-4">
    <div class="flex-shrink-0">
      <GnTag :label="`${score}/${totalAttempts}`" />
    </div>
    <p> {{ questionText }}</p>
  </div>
</template>

<script>
export default {
  name: "QuestionAccuracyItem",
  props: {
    score: {
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
  flex: 1;
  min-width: 0;
  color: #b5b3b3;
  font-size: 10px;
}
</style>
