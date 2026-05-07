<template>
  <div class="flex col-center gap-4 w-75 p-4 h-auto">
    <div class="flex w-100" style="height: 40px">
      <FormInputField :text="`${deckNameText}`" />
    </div>

    <div class="flex w-100" style="height: 100px;">
      <FormTextField :description="`${deckDescriptionText}`" />
    </div>

    <div class="flex col-center gap-2 w-100">
      <div class="flex row-center gap-2 w-100" style="height: 40px">
        <FormInputField
          placeholder="Add a tag to the deck"
          :text="newTag"
          @update:text="newTag = $event"
        />
        <font-awesome-icon icon="fa-solid fa-plus" @click="addTag" />
      </div>
    </div>

    <div class="flex wrap items-start gap-2 row g-1" style="min-height: 15px;">
      <Tag
        v-for="(item, index) in tags"
        :key="index"
        :title="item"
        height="10px"
      />
    </div>

    <br> </br>
    <div class="flex row-center gap-4" style="height: 35px">
      <SecondaryButton :onClickHandler="goBack" text="Discard" />
      <PrimaryButton :onClickHandler="submitHandler" :text="`${submitText}`" />
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
