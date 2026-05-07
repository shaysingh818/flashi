<template>
  <div class="bottom-actions-bar">
    <SecondaryButton :onClickHandler="showDropdown" text="Skip" />
    <PrimaryButton :onClickHandler="showDropdown" text="Submit" />

  </div>
</template>

<script>
import PrimaryButton from '../../components/shared/buttons/PrimaryButton.vue';
import SecondaryButton from '../../components/shared/buttons/SecondaryButton.vue';

export default {
  name: "QuestionActionsBar",
  components: {
    PrimaryButton,
    SecondaryButton
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

<style scoped>

.bottom-actions-bar {
  display: flex; 
  flex-direction: row;
  gap: 1.0rem; 
  width: 100%;
  height: 35px;
  position: fixed;
  background-color: rgb(28 25 23);
  padding: 10px; 
  bottom: 0;
  justify-content: center;
  align-items: center;
  margin-left: -55px; 
}

</style>