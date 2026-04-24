<template>
  <div class="card-content">
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
      type: Number,
      required: true,
    },
  },
  methods: {
    closeDialog() {
      this.$emit("close");
    },
    submitHandler() {
      this.$emit("confirm");
    },
  },
};
</script>

<style scoped>
.card-content {
  display: flex;
  flex-direction: column;
  align-items: stretch;
  text-align: center;
  justify-content: center;
  border-radius: 3%;
  width: 100%;
  height: 100%;
  background-color: rgb(28 25 23);
  padding: 20px;
}

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
