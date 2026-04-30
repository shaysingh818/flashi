<template>
  <div class="container col-center">
    <BackBar text="Deck Name" />

    <!-- question text preview, needs to be certain max width -->
    <div class="container centered">
      <QuestionTextPreview text="How is mitosis important for cell growth, How is mitosis important for cell growth, How is mitosis important for cell growth, How is mitosis important for cell growth" />      
    </div>

    <div class="container centered">
      <OrderedSequencePreview v-if="answerType == 'ordered sequence'" />
      <MultipleChoicePreview v-if="answerType == 'multiple choice'" />
      <OpenEndedPreview v-if="answerType == 'open ended'" placeholderText="Enter answer" />
    </div>    

    
  </div>

  <QuestionActionsBar /> 

</template>

<script>
import BackBar from "../../components/shared/BackBar.vue";
import QuestionTextPreview from "../../components/question/QuestionTextPreview.vue";
import OrderedSequencePreview from "../answer/ordered_sequence/OrderedSequencePreview.vue";
import MultipleChoicePreview from "../answer/multiple_choice/MultipleChoicePreview.vue";
import OpenEndedPreview from "../answer/open_ended/OpenEndedPreview.vue";
import QuestionActionsBar from "./QuestionActionsBar.vue";


export default {
  name: "ViewQuestion",
  components: {
    BackBar,
    QuestionTextPreview,
    OrderedSequencePreview,
    MultipleChoicePreview,
    OpenEndedPreview,
    QuestionActionsBar
  },
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
      questionText: '',
      answerType: 'ordered sequence',
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
