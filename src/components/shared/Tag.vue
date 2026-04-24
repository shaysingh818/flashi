<template>
  <div class="badge">
    <p>{{ title }}</p>
  </div>
</template>

<script>
export default {
  name: "Tag",
  props: {
    title: {
      type: String,
      required: true,
    },
  },
};
</script>

<style scoped>
.badge {
  display: flex;
  justify-content: center;
  align-items: center;
  border-radius: 5px;
  background-color: #f97316;
  padding: 5px;
  white-space: nowrap;
}

p {
  font-weight: normal;
  font-size: 10px;
  color: white;
}
</style>
