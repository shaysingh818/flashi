<template>
  <div class="flex col-center w-100">
    <br />
    <SearchComponent text="Search Sessions" :onClick="search" />
    <div class="flex items-center justify-center">
      <h2>Sessions</h2>
    </div>
    <SessionListView />
    <HoverButton icon="fa-solid fa-plus" :onClickHandler="createSession" />
  </div>
</template>

<script>
import SearchComponent from "../components/shared/forms/SearchComponent.vue";
import SessionListView from "../components/session/SessionListView.vue";
import HoverButton from "../components/shared/buttons/HoverButton.vue";

export default {
  components: {
    SearchComponent,
    SessionListView,
    HoverButton,
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
h2 {
  color: white;
}
</style>
