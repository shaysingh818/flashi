<template>
  <div class="flex centered-all p-4 gap-2 w-100" style="margin-left: 40px; height: 35px; background-color: rgb(28 25 23);">
    <div class="flex row-center w-100" style="margin-right: 55px;">
      <TabNavigationItem icon="fa-regular fa-calendar" text="Session Review" :isSelected="selected === 'Session Review'" @select="selectTab('Session Review')"/>
      <TabNavigationItem icon="fa-solid fa-list-check" text="Decks" :isSelected="selected === 'Decks'" @select="selectTab('Decks')"/>
      <TabNavigationItem icon="fa-regular fa-chart-bar" text="Statistics" :isSelected="selected === 'Statistics'" @select="selectTab('Statistics')"/>
    </div>
  </div>
</template>

<script>
import TabNavigationItem from './TabNavigationItem.vue';

export default {
  name: "TabNavigationBar",
  components: { TabNavigationItem },
  props: {},
  data() {
    return {
      selected: 'Decks',
    };
  },
  methods: {
    showDropdown() {
      console.log("Show dropdown", this.isDropdownOpen);
      this.isDropdownOpen = !this.isDropdownOpen;
    },
    selectTab(tab) {
      this.selected = tab;
      this.$emit('select', tab);  // bubbles up to the parent
    },
  },
};
</script>

<style scoped>

p {
  color: white; 
  font-size: 13px; 
}

</style>