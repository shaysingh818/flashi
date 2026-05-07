<template>
  <button @click="handleClick"> 
    <font-awesome-icon :icon="`${icon}`" class="icon-style" />
  </button>
</template>

<script>
export default {
  name: 'PlayButton',
  props: {
    icon: {
      type: String,
      required: true,
    },
    onClickHandler: {
      type: Function,
      required: true,
    },
  },
  methods: {
    handleClick() {
      this.onClickHandler();
    }
  }
} 
</script>

<style scoped>

button {   
  padding: 2px 2px;
  font-size: 13px;
  background-color: #F97316;
  color: white;
  border: none;
  border-radius: 50%;
  height: 30px;
  width: 30px;
  cursor: pointer;
}

.icon-style {
  color: white; 
}

</style>
