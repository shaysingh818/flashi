<template>

  <div class="nav-item">
    <font-awesome-icon :icon="`fa-regular fa-${icon}`" class="icon-style" />
    <p> {{ navTitle }} </p>
  </div>

</template>

<script>
export default {
  name: 'NavigationItem',
  props: {
    icon: {
      type: String,
      required: true,
    },
    navTitle: {
      type: String,
      required: true,
    },
  },
} 
</script>

<style scoped>

  .nav-item {
    display: flex;
    flex-direction: column; 
    justify-content: center; 
    align-items: center;
    padding: 5px; 
  }

  .icon-style {
    color: #F97316;
  } 

  p {
    color: #B5B3B3;
    font-size: 10px; 
  }

</style>

