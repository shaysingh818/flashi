<template>
  <div @click.stop="handleClick" class="flex row items-center gap-2">
    <font-awesome-icon :icon="`${icon}`" />
    <p>{{ text }}</p>
  </div>
</template>

<script>
export default {
  name: "DropDownItem",
  props: {
    icon: {
      type: String,
      required: true,
    },
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

.dropdown-item {
  display: flex;
  flex-direction: row;
  justify-content: left;
  align-items: left;
  height: 30px;
  gap: 0.5rem;
}

.dropdown-item:hover {
  background-color: gray;
}

.leading-icon {
  display: flex;
  flex: 0 0 25%;
  justify-content: center;
  align-items: center;
  height: 100%;
}

.content {
  display: flex;
  flex: 1 1 auto;
  justify-content: left;
  align-items: center;
  width: 75%;
}

svg {
  color: #f97316;
  height: 20px;
  width: 20px;
}

p {
  color: #b5b3b3;
  font-size: 10px;
}
</style>
