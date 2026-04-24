<template>
  <div class="create-subject-header">
    <div class="header-text">
      <h1>{{ header }}</h1>
      <p>{{ description }}</p>
    </div>
  </div>
</template>

<script>
export default {
  name: "FormHeader",
  props: {
    header: {
      type: String,
      required: true,
    },
    description: {
      type: String,
      required: true,
    },
  },
};
</script>

<style scoped>
.create-subject-header {
  width: 85%;
  border-radius: 10px;
  background-color: rgb(28 25 23);
}

.header-text {
  padding: 10px;
}

h1 {
  font-size: 25px;
  margin-bottom: 0;
}

p {
  color: #b5b3b3;
  font-size: 15px;
  margin-top: 1;
}
</style>
