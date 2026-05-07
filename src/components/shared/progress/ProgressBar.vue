<template>
  <div class="progress-bar">
    <div class="progress-bar-progress" :style="{ width: clampedProgress + '%' }"></div>
  </div>
</template>

<script>


export default {
  name: "ProgressBar",
  props: {
    progress: {
      type: Number,
      default: 0,
    },
  },
  components: { },
  computed: {
    clampedProgress() {
      return Math.min(100, Math.max(0, this.progress));
    },
  },
  methods: {},
};
</script>

<style scoped>

.progress-bar {
  height: 7px; 
  border-radius: 20px; 
  background-color: #3D3D3D;
  width: 75%; 
}

.progress-bar-progress {
  height: 7px; 
  border-radius: 20px; 
  background-color: #F97316;
  justify-content: left; 
}

</style>
