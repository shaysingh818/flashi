<template>
  <div 
    class="flex col-center w-100 h-100 rounded-2 text-center p-2"
    style="background-color: rgb(28 25 23);">
    <h1>{{ metric }}</h1>
    <h2>{{ numeric }}</h2>
  </div>
</template>

<script>
export default {
  name: "MetricCard",
  props: {
    metric: {
      type: String,
      required: true,
    },
    numeric: {
      type: String,
      required: true,
    },
  },
};
</script>

<style scoped>

h1 {
  color: white;
  font-weight: bold;
  font-size: 17px;
  margin-bottom: 0;
}

h2 {
  margin-top: 0;
  color: #f97316;
  font-weight: bold;
  font-size: 25px;
}
</style>