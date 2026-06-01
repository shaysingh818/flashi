<template>
  <div class="bottom-actions-bar">
    <SecondaryButton @click="showDropdown" text="Skip" />
    <PrimaryButton @click="props.submitHandler" text="Testing" />

  </div>
</template>

<script setup>
import PrimaryButton from '../shared/PrimaryButton.vue';
import SecondaryButton from '../shared/SecondaryButton.vue';

const props = defineProps({
  submitHandler: {
    type: Function,
    required: true,
  },
});
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
