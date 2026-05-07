<template>
  <div class="flex w-75 p-4" style="height: 100px;">
   <FormTextField :description="placeholderText" />
  </div>

</template>

<script>
import FormTextField from '../../../components/shared/forms/FormTextField.vue';

export default {
  name: "ViewQuestion",
  components: {
    FormTextField
  },
  props: {
    placeholderText: {
      type: String,
      required: true,
    },
  },
  data() {
    return {
      questionText: '',
      answerType: 'multiple choice',
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