<template>
  <div class="bottom-actions-bar">
    <GnButton :disabled="false" variant="secondary" @gn-click="props.previousHandler">
        Previous
    </GnButton>
   
    <GnButton :disabled="false" variant="primary" @gn-click="props.updateHandler">
        Update
    </GnButton>
    
    <GnButton :disabled="false" variant="secondary" @gn-click="props.nextHandler">
        Next
    </GnButton>
  </div>
</template>


<script setup>

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
  bottom: 0;
  justify-content: center;
  align-items: center;
  margin-left: -55px; 
  padding: 25px;
}

</style>
