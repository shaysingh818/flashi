<template>
  <div class="back-bar">
    <div class="back-button">
      <font-awesome-icon icon="fa-solid fa-arrow-left" @click="goBack" />
    </div>

    <div class="text-section">
      <h1>Subject Name</h1>
      <p>Last Updated: 04/02/2026</p>
    </div>

    <div class="trailing">
      <font-awesome-icon icon="fa-regular fa-pen-to-square" @click="goBack" />
    </div>
  </div>
</template>

<script>
export default {
  name: "BackBar",
  props: {
    text: {
      type: String,
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
    goBack() {
      this.$router.go(-1);
    },
  },
};
</script>

<style scoped>
.back-bar {
  display: flex;
  flex-direction: row;
  background-color: rgb(28 25 23);
  width: 100%;
  height: 60px;
  border-radius: 5px;
  gap: 1rem;
}

.back-button {
  display: flex;
  justify-content: center;
  align-items: center;
  width: 10%;
}

svg {
  color: #f97316;
  width: 20px;
  height: 20px;
}

.text-section {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  width: 75%;
  height: 100%;
  white-space: nowrap;
  overflow: hidden;
  gap: 0.1rem;
}

.trailing {
  display: flex;
  justify-content: center;
  align-items: center;
  width: 10%;
}

h1 {
  font-size: 15px;
  margin-bottom: 0;
  font-weight: bold;
}

p {
  color: #b5b3b3;
  font-size: 10px;
  margin-top: 0;
}
</style>
