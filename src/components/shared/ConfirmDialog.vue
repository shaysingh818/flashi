<template>
  <div class="dialog-overlay">
    <div class="dialog-box">
      <div class="dialog-text">
        <p>{{ text }}</p>
      </div>
      <div class="button-dialog">
        <SecondaryButton :onClickHandler="closeDialog" text="No" />
        <PrimaryButton :onClickHandler="submitHandler" text="Yes" />
      </div>
    </div>
  </div>
</template>

<script>
import PrimaryButton from "../../components/shared/PrimaryButton.vue";
import SecondaryButton from "../../components/shared/SecondaryButton.vue";

export default {
  name: "ConfirmDialog",
  components: {
    PrimaryButton,
    SecondaryButton,
  },
  props: {
    text: {
      type: String,
      required: true,
    },
  },
  methods: {
    closeDialog() {
      this.$emit("close");
    },
    submitHandler() {
      this.$emit("confirm");
    },
  },
};
</script>

<style scoped>
.dialog-overlay {
  position: fixed;
  inset: 0;
  background: rgba(0, 0, 0, 0.5);
  display: flex;
  justify-content: center;
  align-items: center;
  z-index: 1000;
}

.dialog-box {
  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: center;
  padding: 10px;
  background-color: rgb(28 25 23);
  border-radius: 5px;
  width: 50%;
}

.button-dialog {
  display: flex;
  flex-direction: row;
  justify-content: center;
  align-items: center;
  gap: 0.5rem;
  padding: 10px;
  height: 30px;
}

p {
  color: #b5b3b3;
  font-size: 12px;
}
</style>
