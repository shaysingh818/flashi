<template>
  <div class="flex centered-all" style="background-color: rgb(28 25 23); margin: 20px; border-radius: 10px; height: 75%;">
    <p class="flex p-6">{{ text }} </p>
  </div>

</template>

<script>
export default {
  name: "QuestionTextPreview",
  components: {},
  props: {
    text: {
      type: String,
      required: true,
    },
  },
  data() {
    return {
      tags: [],
      selectedOption: "",
    };
  },
  methods: {
    goBack() {
      this.$router.go(-1);
    },
  },
};
</script>

<style scoped>

p {
  color: #b5b3b3; 
  font-size: 15px; 
}

</style>