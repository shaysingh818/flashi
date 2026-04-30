<template>
  <div class="container col-center g-1 w-75">
    <div class="container row-center g-1" style="height: 35px">
      <FormInputField
        placeholder="Add keyword for open ended answer"
        @update:text="newSequence = $event"
      />
      <font-awesome-icon icon="fa-solid fa-plus" @click="addSequence" />
    </div>

    <div class="container col-center">
      <OrderedSequenceFormItem
        v-for="(item , index) in sequences"
        :key="index"
        :index="index+1"
        :content="item"
      />
    </div>
  </div>
</template>

<script>
import FormInputField from "../../../components/shared/FormInputField.vue";
import FormTextField from "../../../components/shared/FormTextField.vue";
import Tag from "../../../components/shared/Tag.vue";
import OrderedSequenceFormItem from "./OrderedSequenceFormItem.vue";

export default {
  name: "CreateOrderedSequenceForm",
  components: {
    FormInputField,
    OrderedSequenceFormItem,
  },
  data() {
    return {
      sequences: [],
      newSequence: "",
    };
  },
  methods: {
    addSequence() {
      this.sequences.push(this.newSequence);
      this.newSequence = "";
    },
  },
};
</script>

<style scoped>
.number-index {
  color: #f97316;
  font-size: 12px;
}

.indice-content {
  color: #b5b3b3;
  font-size: 12px;
}

svg {
  color: #f97316;
}
</style>
