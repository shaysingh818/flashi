<template>
  <div class="container row-center g-1" style="height: 35px;">
    <p>{{ index }}. </p>
    <FormInputField placeholder="" @update:text="newItem = $event" />
  </div>
</template>

<script>
import FormInputField from "../../../components/shared/FormInputField.vue";

export default {
  name: "OrderedSequencePreviewItem",
  components: { FormInputField },
  props: {
    index: {
      type: Number,
      required: true,
    },
  },
  data() {
    return {
      isDropdownOpen: false,
      selected: null,
      newItem: ''
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
p {
  color: #f97316;
}
</style>