<template>
  <Teleport to="body">
    <div v-if="open" class="dialog-overlay" @click="emit('close')">
      <div class="dialog-box" :style="{ width, height }" @click.stop>
        <slot /> 
      </div>
    </div>
  </Teleport>
</template>

<script setup>
defineProps({
  open:   { type: Boolean, required: true },
  width:  { type: String, default: '50rem' },
  height: { type: String, default: 'auto' },
});
const emit = defineEmits(['close']);
</script>

<style scoped>
.dialog-overlay {
  position: fixed;
  inset: 0;
  display: flex;
  align-items: center;
  justify-content: center;
  background-color: rgba(0, 0, 0, 0.5);
  z-index: 1000;
}
.dialog-box {
  max-width: 90vw;
  max-height: 90vh;
  overflow-y: auto;
  padding: 1.25rem;
  background-color: var(--app-color-background);
  border-radius: 0.5rem;
  box-sizing: border-box;
}
</style>
