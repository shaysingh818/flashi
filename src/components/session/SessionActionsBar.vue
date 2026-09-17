<template>
  <div class="bottom-actions-bar">
    <GnButton :disabled="false" variant="secondary" @gn-click="props.skipHandler">
        Skip
    </GnButton>
   
    <GnButton :disabled="false" variant="primary" @gn-click="props.submitHandler">
        Submit
    </GnButton>
  </div>
</template>


<script setup>
import PrimaryButton from '../../components/shared/buttons/PrimaryButton.vue';
import SecondaryButton from '../../components/shared/buttons/SecondaryButton.vue';

const props = defineProps({
  skipHandler: {
    type: Function,
    required: true,
  },
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
  padding: 25px; 
  bottom: 0;
  justify-content: center;
  align-items: center;
  margin-left: -55px; 
}

</style>
