<template>
    <div class="flex p-2"> 
        <div class="flex flex-col" style="background-color: var(--app-color-surface); border-radius: 10px;">
        <div class="p-6">
            <h1>{{ header }}</h1>
            <p>{{ description }}</p>
        </div>
        </div>
    </div>
</template>

<script setup>
const props = defineProps({
    header: {
      type: String,
      required: true,
    },
    description: {
      type: String,
      required: true,
    },
});
</script>

<style scoped>

h1 {
  font-size: 25px;
  margin-bottom: 0;
  color: var(--app-color-text);
}

p {
  color: var(--app-color-text-muted); 
  font-size: 15px;
  margin-top: 1;
}
</style>
