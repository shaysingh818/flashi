<template>
  <div class="flex row w-100 rounded-2 gap-2" style="background-color: rgb(28, 25,23); height: 60px;">
    <div class="flex centered-all" style="width: 10%">
      <font-awesome-icon icon="fa-solid fa-arrow-left" @click="goBack" />
    </div>

    <div class="flex col-center w-75 h-100">
      <h1>{{ text }}</h1>
      <p>Last Updated: 04/02/2026</p>
    </div>

    <div class="flex centered-all" style="width: 10%;">

      <font-awesome-icon
        v-for="(item, index) in actions"
        :key="index"
        :icon="item.icon" 
        @click="item.handler()" 
      />
      
      <!-- <font-awesome-icon icon="fa-regular fa-pen-to-square" @click="goBack" /> -->
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
    actions: {
      type: Array,
      required: false,
      default: () => [],
    }
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

svg {
  color: #f97316;
  width: 20px;
  height: 20px;
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
