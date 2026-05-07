<template>
  <div 
    class="flex row-center gap-2 w-100" 
    :style="{ borderBottom: isSelected ? '2px solid #F97316' : '2px solid transparent' }"
    @click="$emit('select')"
    style="cursor: pointer;"
    >
    <font-awesome-icon :icon="`${icon}`" class="icon-style" />
    <p>{{ text }}</p>      
  </div>

</template>

<script>


export default {
  name: "TabNavigationItem",
  components: { },
  props: {
    icon: {
      type: String,
      required: true,
    },
    text: {
      type: String,
      required: true,
    },
    isSelected: { 
      type: Boolean, 
      default: false 
    },
  },
  emits: ['select'],
  data() {
    return {
      isDropdownOpen: false,
    };
  },
  methods: {},
};
</script>

<style scoped>

p {
  color: white;
  font-size: 13px; 
}

.icon-style {
  color: #F97316;
} 

</style>