<template>
  <div
    class="flex col-center gap-4 w-75 p-4"
    style="height: auto"
  >
    <div class="flex w-100" style="height: 40px">
      <FormInputField :text="`${deckName}`" />
    </div>

    <div class="flex w-100" style="height: 100px;">
      <FormTextField :description="`${deckDescription}`" />
    </div>

    <div class="container centered align-center col g-1">
      <div class="container row-center g-1" style="height: 40px">
        <FormInputField
          placeholder="Update tags for deck"
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
      <PrimaryButton :onClickHandler="submitHandler" text="Update" />
    </div>
  </div>
</template>

<script>
import FormInputField from "../../components/shared/forms/FormInputField.vue";
import FormTextField from "../../components/shared/forms/FormTextField.vue";
import PrimaryButton from "../../components/shared/buttons/PrimaryButton.vue";
import SecondaryButton from "../../components/shared/buttons/SecondaryButton.vue";
import Tag from "../shared/Tag.vue";

export default {
  name: "EditDeckForm",
  components: {
    FormInputField,
    FormTextField,
    PrimaryButton,
    SecondaryButton,
    Tag,
  },
  props: {
    deckName: {
      type: String,
      required: true,
    },
    deckDescription: {
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
