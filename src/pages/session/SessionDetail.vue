<template>
  <div class="flex col-center w-100">
    <BackBar text="Session Name" />
    <TabNavigationBar @select="activeTab = $event" />
    <SessionDecks v-if="activeTab === 'Decks'" />
    <SessionReview v-if="activeTab === 'Session Review'" />
    <SessionStatistics v-if="activeTab === 'Statistics'" />
  </div>
</template>

<script>
import TabNavigationBar from '../../components/shared/navigation/TabNavigationBar.vue';
import BackBar from '../../components/shared/navigation/BackBar.vue';
import SessionDecks from './SessionDecks.vue';
import SessionReview from './SessionReview.vue';
import SessionStatistics from './SessionStatistics.vue';

export default {
  name: "SessionDetail",
  components: { 
    TabNavigationBar,
    BackBar,
    SessionDecks,
    SessionReview,
    SessionStatistics
  },
  data() {
    return { activeTab: 'Decks' }
  },
  methods: {
    async search() {
      console.log("test"); 
    },
    async createSubject() {
      this.$router.push({ path: "/subjects/create"})
    }
  },
}
</script>

<style scoped>

</style>
