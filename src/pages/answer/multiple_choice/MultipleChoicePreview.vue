<template>
  <div class="container col-center g-1 w-75 p-10">
    <MultipleChoicePreviewItem
      v-for="(item, index) in items"
      :key="index"
      :text="item"
    />
  </div> 

</template>

<script>
import MultipleChoicePreviewItem from './MultipleChoicePreviewItem.vue';


export default {
  name: "MultipleChoicePreview",
  components: { MultipleChoicePreviewItem },
  props: {},
  data() {
    return {
      isDropdownOpen: false,
      selected: null,
      items: [
        "option 1, go through the blank", 
        "option 2, go through the ranks with anks with danks", 
        "option 3"
      ],
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

</style>