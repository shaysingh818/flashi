<template>
  <Teleport to="body">
    <div v-if="open" class="dialog-overlay" @click="emit('close')">
      <div class="dialog-box" :style="{ width, height }" @click.stop>

        <div class="dialog-header">
          <font-awesome-icon
            v-if="icon"
            :icon="icon"
            class="dialog-icon"
            :style="{ color: iconColor }"
          />
          <h2 class="dialog-message">{{ dialogMessage }}</h2>
        </div>

        <div v-if="showExplanation" class="dialog-body">
          <p v-if="explanation" class="dialog-explanation">{{ explanation }}</p>
          <p v-else class="dialog-explanation dialog-explanation-empty">
            No explanation provided for this question.
          </p>
          <slot />
        </div>

        <div class="dialog-actions">
          <GnButton :disabled="false" variant="primary" @gn-click="emit('close')">
            Continue
          </GnButton>
        </div>

      </div>
    </div>
  </Teleport>
</template>

<script setup>
defineProps({
  open:          { type: Boolean, required: true },
  width:         { type: String, default: '50rem' },
  height:        { type: String, default: 'auto' },
  icon:          { type: String, default: '' },
  iconColor:     { type: String, default: 'var(--app-color-accent)' },
  dialogMessage: { type: String, default: '' },
  explanation:   { type: String, default: '' },
  showExplanation: { type: Boolean, default: true },
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
  display: flex;
  flex-direction: column;
  gap: 1rem;
  max-width: 90vw;
  max-height: 90vh;
  padding: 1.25rem;
  background-color: var(--app-color-background);
  color: var(--app-color-text);
  border-radius: 0.5rem;
  box-sizing: border-box;
}
.dialog-header {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 0.75rem;
}
.dialog-icon {
  font-size: 3rem;
}
.dialog-message {
  font-size: 1.5rem;
  font-weight: 600;
}
.dialog-body {
  flex: 1;
  min-height: 0;
  overflow-y: auto;
  display: flex;
  flex-direction: column;
  align-items: center;
}
.dialog-explanation {
  margin: auto;
  white-space: pre-wrap;
  text-align: center;
}
.dialog-explanation-empty {
  color: var(--app-color-text-muted);
  text-align: center;
}
.dialog-actions {
  display: flex;
  justify-content: center;
}
</style>
