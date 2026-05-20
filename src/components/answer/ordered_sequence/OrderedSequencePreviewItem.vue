<template>
  <div class="flex row-center gap-2 w-100" style="height: 35px;">
    <p>{{ index }}. </p>
    <FormInputField :placeholder="text" @update:text="newItem = $event" />
  </div>
</template>

<script setup>
import { ref } from "vue";
import FormInputField from "../../../components/shared/forms/FormInputField.vue";

const props = defineProps({
  index: {
    type: Number,
    required: true,
  },
  text: {
    type: String,
    required: true,
  },
});

const newItem = ref(''); 


</script>

<style scoped>
p {
  color: #f97316;
}
</style>