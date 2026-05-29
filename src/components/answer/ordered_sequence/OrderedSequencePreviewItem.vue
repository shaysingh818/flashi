<template>
  <div v-if="props.visible === false" class="flex row-center gap-2 w-100" style="height: 35px;">
    <p style="color: #f97316;">{{ index }}. </p>
    <FormInputField 
      :text="text" 
      :placeholder="text" 
      @update:text="newItem = $event; emitText()"
    />
  </div>

  <div v-if="props.visible === true" class="flex row-center gap-4 w-100">

    <div class="flex row-center gap-2 w-90" style="height: 35px;">
      <p style="color: #f97316;">{{ index }}. </p>
      <FormInputField 
        :text="text" 
        :placeholder="text" 
        @update:text="newItem = $event; emitText()"
      />
    </div>

    <div class="flex row-center fw-10" style="height: 35px;">
      <font-awesome-icon icon="fa-solid fa-x" style="color: #f97316"  @click="deleteSequence" />
    </div>

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
  visible: {
    type: Boolean,
    default: () => false,
  },
});

const newItem = ref('');
const emit = defineEmits(["update:text", "update:index", "update:item", "update:delete"]);


function emitText() {
  emit('update:text', newItem.value);
}

function deleteSequence() {
  emit('update:delete', props.index);
}
</script>
