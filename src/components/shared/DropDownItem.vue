<template>
  <div @click.stop="editSubject" class="dropdown-item">
    <font-awesome-icon :icon="`${icon}`" class="dropdown-icon-style" />
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
.dropdown-item {
  display: flex;
  flex-direction: row;
  justify-content: left;
  align-items: left;
  gap: 0.5rem;
}

.dropdown-icon-style {
  color: #f97316;
}
</style>
