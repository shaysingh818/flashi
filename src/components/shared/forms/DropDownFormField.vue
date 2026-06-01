<template>
  <div class="flex col-center w-100 g-1">
    <div class="flex row-center gap-4">
      <div class="flex">
        <label>Choose an answer type:</label>
      </div>
      <div
        class="flex centered-all w-50"
        style="background-color: rgb(28 25 23); border-radius: 10px;"
        @click="showDropdown"
      >
        <p>{{ selected || "Select an option" }}</p>
      </div>
    </div>

    <Teleport to="body">
      <div
        v-if="isDropdownOpen"
        style="background-color: rgb(28 25 23); border-radius: 10px"
        class="flex col-center centered-overlay w-50 p-4 g-1"
      >
        <div class="flex centered-all">
          <h1>Choose Answer Type</h1>
        </div>

        <div class="flex col-center">
          <p
            v-for="(item, index) in options"
            :key="index"
            class="dropdown-text"
            :class="{ selected: selected === item }"
            @click="selectItem(item)"
          >
            {{ item }}
          </p>
        </div>

        <br/>
        <div class="flex row-center gap-4 p-10" style="height: 30px">
          <SecondaryButton @click="showDropdown" text="Discard" />
          <PrimaryButton @click="showDropdown" text="Submit" />
        </div>
      </div>
    </Teleport>
  </div>
</template>

<script>
import DropDownItem from "../navigation/DropDownItem.vue";
import PrimaryButton from "../buttons/PrimaryButton.vue";
import SecondaryButton from "../buttons/SecondaryButton.vue";

export default {
  name: "DropDownFormField",
  components: { DropDownItem, PrimaryButton, SecondaryButton },
  props: {
    title: {
      type: String,
      required: true,
    },
    options: {
      type: Array,
      required: false,
      default: () => [],
    },
  },
  data() {
    return {
      isDropdownOpen: false,
      selected: null,
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

label {
  padding: 10px;
  color: #b5b3b3;
  font-size: 15px;
}

p {
  color: white;
  font-size: 15px;
}

h1 {
  font-size: 20px;
  font-weight: bold;
}

.dropdown-text {
  font-size: 12px;
  border-radius: 10px;
  padding: 5px;
}

.dropdown-text:hover {
  background-color: gray;
}

.selected {
  background-color: #f97316;
}
</style>
