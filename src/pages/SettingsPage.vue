<template>
  <div class="flex col-center w-100">

    <div class="flex items-left w-90">
      <h1> General </h1>
    </div>
    
    <div class="flex col items-left w-100">
      <SettingsToggle text="Dark mode, this will use the default dark theme appearence for the application" />
      <SettingsToggle text="Allow hosting of study sessions to other users on a LAN network" />
      <SettingsToggle text="Allow random generation of decks" />
    </div>
  </div>
</template>


<script>
import SettingsToggle from '../components/shared/SettingsToggle.vue';

export default {
  components: {
    SettingsToggle
  },
  methods: {
    async search() {
      console.log("test");
    },
    async createSession() {
      this.$router.push({ path: "/sessions/create" });
    },
  },
};
</script>

<style scoped>

</style>
