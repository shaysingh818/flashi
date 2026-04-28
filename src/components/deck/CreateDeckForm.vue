<template>
  <div
    class="container col-centered align-center col g-1 w-75 p-10"
    style="height: auto"
  >
    <div class="container" style="height: 40px">
      <FormInputField :text="`${deckNameText}`" />
    </div>

    <div class="container">
      <FormTextField :description="`${deckDescriptionText}`" />
    </div>

    <div class="container centered align-center col g-1">
      <div class="container row-center g-1" style="height: 40px">
        <FormInputField
          placeholder="Add a tag to the deck"
          :text="newTag"
          @update:text="newTag = $event"
        />
        <font-awesome-icon icon="fa-solid fa-plus" @click="addTag" />
      </div>
    </div>

    <div
      class="container centered align-left row g-1"
      style="min-height: 15px; flex-wrap: wrap; align-items: flex-start"
    >
      <Tag
        v-for="(item, index) in tags"
        :key="index"
        :title="item"
        height="10px"
      />
    </div>

    <br> </br>
    <div class="container row-center g-1" style="height: 35px">
      <SecondaryButton :onClickHandler="goBack" text="Discard" />
      <PrimaryButton :onClickHandler="submitHandler" :text="`${submitText}`" />
    </div>
  </div>
</template>

<script>
import FormInputField from "../../components/shared/FormInputField.vue";
import FormTextField from "../../components/shared/FormTextField.vue";
import PrimaryButton from "../../components/shared/PrimaryButton.vue";
import SecondaryButton from "../../components/shared/SecondaryButton.vue";
import Tag from "../shared/Tag.vue";

export default {
  name: "CreateDeckForm",
  components: {
    FormInputField,
    FormTextField,
    PrimaryButton,
    SecondaryButton,
    Tag,
  },
  props: {
    deckNameText: {
      type: String,
      required: true,
    },
    deckDescriptionText: {
      type: String,
      required: true,
    },
    deckTagText: {
      type: String,
      required: true,
    },
    submitText: {
      type: String,
      required: true,
    },
    submitHandler: {
      type: Function,
      required: true,
    },
  },
  data() {
    return {
      tags: [],
      newTag: "",
    };
  },
  methods: {
    goBack() {
      this.$router.go(-1);
    },
    addTag() {
      this.tags.push(this.newTag);
      this.newTag = "";
    },
  },
};
</script>

<style scoped>
svg {
  color: #f97316;
}


</style>
