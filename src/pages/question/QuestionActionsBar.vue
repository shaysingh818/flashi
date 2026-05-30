<template>
  <div class="bottom-actions-bar">
    <SecondaryButton :onClickHandler="props.previousHandler" text="Previous" />
    <PrimaryButton :onClickHandler="props.updateHandler" text="Update" />
    <SecondaryButton :onClickHandler="props.nextHandler" text="Next" />
  </div>
</template>


<script setup>
import PrimaryButton from '../../components/shared/buttons/PrimaryButton.vue';
import SecondaryButton from '../../components/shared/buttons/SecondaryButton.vue';

const props = defineProps({
  previousHandler: {
    type: Function,
    required: true,
  },
  updateHandler: {
    type: Function,
    required: true,
  },
  nextHandler: {
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
