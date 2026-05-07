<template>
<div class="flex row-center gap-2">

  <p class="flex fw-75">{{ text }}</p>
  
  <label class="switch">
    <input type="checkbox" checked>
    <span class="slider round"></span>
  </label>
</div>

</template>

<script>
export default {
  name: "SettingsToggle",
  components: {  },
  props: {
    text: {
      type: String,
      required: true,
    },
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

.switch {
  position: relative;
  display: inline-block;
  width: 55px;
  height: 30px;
}

.switch input { 
  opacity: 0;
  width: 0;
  height: 0;
}

.slider {
  position: absolute;
  cursor: pointer;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background-color: rgb(28 25 23);
  -webkit-transition: .4s;
  transition: .4s;
}

.slider:before {
  position: absolute;
  content: "";
  height: 23px;
  width: 23px;
  left: 4px;
  bottom: 4px;
  background-color: white;
  -webkit-transition: .4s;
  transition: .4s;
}

input:checked + .slider {
  background-color: #f97316;
}

input:focus + .slider {
  box-shadow: 0 0 1px rgb(28 25 23);
}

input:checked + .slider:before {
  -webkit-transform: translateX(26px);
  -ms-transform: translateX(26px);
  transform: translateX(26px);
}


.slider.round {
  border-radius: 34px;
}

.slider.round:before {
  border-radius: 50%;
}

p {
  color: #b5b3b3; 
  font-size: 12px; 
}

</style>