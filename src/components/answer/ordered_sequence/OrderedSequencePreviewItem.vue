<template>
  <div v-if="props.visible === false" class="flex flex-row items-center gap-2">
    <p style="color: var(--app-color-accent);">{{ index }}. </p>
    <GnFormInputField
      class="flex-1"
      @update:model-value="newItem = $event; emitText()"
      placeholder="Enter answer"
      backgroundColor="var(--app-color-surface)" 
      borderColor="transparent" 
      :disabled="false" 
      :placeholder="text" 
      textColor="var(--app-color-accent)" />

  </div>

  <div v-if="props.visible === true" class="flex row-center gap-">

    <div class="flex flex-row items-center gap-2 w-full">
        <p style="color: var(--app-color-accent)">{{ index }}. </p>
        <GnFormInputField
            class="flex-1"
            @update:model-value="newItem = $event; emitText()"
            :model-value="text"
            backgroundColor="var(--app-color-surface)" 
            borderColor="transparent" 
            :disabled="false" 
            :placeholder="text" 
            textColor="var(--app-color-accent)" />
    </div>

    <div class="flex">
        <GnIconButton
            label="Remove step"
            variant="ghost"
            size="sm"
            color="var(--app-color-accent)"
            @gn-click="deleteSequence"
            >
                <font-awesome-icon icon="fa-solid fa-x" />
        </GnIconButton>
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
