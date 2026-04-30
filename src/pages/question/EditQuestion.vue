<template></template>

<script>


export default {
  name: "EditQuestion",
  components: { },
  props: {
    title: {
      type: String,
      required: true,
    },
    options: {
      type: Array,
      required: false,
      default: () => [],
    },
  },
  data() {
    return {
      isDropdownOpen: false,
      selected: null,
    };
  },
  methods: {
    showDropdown() {
      console.log("Show dropdown", this.isDropdownOpen);
      this.isDropdownOpen = !this.isDropdownOpen;
    },
    selectItem(item) {
      this.selected = item;
      this.$emit("update:title", item); // add this
    },
  },
};
</script>

<style scoped></style>
