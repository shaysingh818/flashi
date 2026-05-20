<template>
  <label for="item">
    <input type="checkbox" :id="text" :name="text" value="yes" :checked="isSelected" />
    {{ text }}
  </label>

</template>

<script>

export default {
  name: "MultipleChoicePreviewItem",
  props: {
    text: {
      type: String,
      required: true,
    },    
    isSelected: {
      type: Boolean,
      required: true,
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

<style scoped>

label {
  display: flex;
  padding: 10px;
  color: #b5b3b3;
  align-items: center;
  justify-content: left;
  width: 100%;
  gap: 1rem;
  font-size: 12px;
}

input[type="checkbox"] {
  appearance: none;
  width: 25px;
  height: 25px;
  background-color: rgb(28 25 23);
  border-radius: 2px;
}

input[type="checkbox"]:checked {
  background-color: #f97316;
  border-color: #007bff;
}

svg {
  color: #f97316;
}

</style>