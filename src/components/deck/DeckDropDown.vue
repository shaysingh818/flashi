<template>
  <div class="dropdown-content">
    <DropDownItem icon="fa-solid fa-pen-to-square" text="Edit Deck" />
    <DropDownItem icon="fa-solid fa-x" text="Delete Deck" />
  </div>
</template>

<script>
import DropDownItem from "../shared/DropDownItem.vue";

export default {
  name: "DeckDropDown",
  components: { DropDownItem },
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
.dropdown-content {
  position: absolute;
  display: flex;
  flex-direction: column;
  border-radius: 4px;
  padding: 10px;
  gap: 0.5rem;
  background-color: rgb(28 25 23);
  transform: translateX(50%) translateY(80%);
  border: 1px solid white;
  width: 150px;
}
</style>
