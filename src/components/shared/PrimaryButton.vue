<template>
  <button @click="handleClick" type="button">{{ text }}</button>
</template>

<script>
export default {
  name: "PrimaryButton",
  props: {
    text: {
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
    },
  },
};
</script>

<style scoped>
button {
  border-radius: 5px;
  width: 100px;
  height: 100%;
  background-color: #f97316;
  color: white;
  border: none;
  font-weight: bold;
}

button:focus {
  outline: none; /* removes it entirely */
}

button:focus-visible {
  outline: 2px solid blue; /* Custom accessible ring */
  outline-offset: 2px;
}
</style>
