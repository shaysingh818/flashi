<template>
  <div class="flex col-center w-100">
    <BackBar text="Session Name" />
    <TabNavigationBar @select="activeTab = $event" />
    <SessionDecks v-if="activeTab === 'Decks'" :sessionId="sessionId" />
    <SessionReview v-if="activeTab === 'Session Review'" :sessionId="sessionId" />
    <SessionStatistics v-if="activeTab === 'Statistics'" :sessionId="sessionId"/>
  </div>
</template>

<script setup>
import { ref } from "vue";
import { useRouter, useRoute } from "vue-router";
import TabNavigationBar from '../../components/shared/navigation/TabNavigationBar.vue';
import BackBar from '../../components/shared/navigation/BackBar.vue';
import SessionDecks from './SessionDecks.vue';
import SessionReview from './SessionReview.vue';
import SessionStatistics from './SessionStatistics.vue';

const route = useRoute();
const router = useRouter();
const activeTab = ref('Decks');
const sessionId = route.params.id;
</script>
