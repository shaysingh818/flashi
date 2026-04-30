<template>
  <div class="container col-center g-1 w-75 p-10">
    <OrderedSequencePreviewItem
      v-for="(item, index) in items"
      :key="index"
      :index="index+1"
    />
  </div>
</template>

<script>
import OrderedSequencePreviewItem from "./OrderedSequencePreviewItem.vue";

export default {
  name: "OrderedSequencePreview",
  components: { OrderedSequencePreviewItem },
  props: {
    questionText: {
      type: String,
      required: true,
    },
    answerType: {
      type: String,
      required: true,
    },
  },
  data() {
    return {
      isDropdownOpen: false,
      selected: null,
      items: [1, 2, 3, 4, 5, 6],
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
