<template>
  <div class="flex flex-col">
      
    <GnBackBar 
        title="Session Name"
        subtitle="Last updated 04/02/2026"
        textColor="var(--app-color-text)"
        color="var(--app-color-accent)"
        backgroundColor="var(--app-color-surface)"
        borderColor="var(--app-color-surface)"
        @gn-back="goBack">
        <font-awesome-icon icon="fa-solid fa-arrow-left" />  
    </GnBackBar>
    
    <GnTabNavigationBar backgroundColor="var(--app-color-surface)" v-model="activeTab" v-slot="{ active, select }">
        
        <GnTabNavigationItem
          label="Session Review"
          :selected="activeTab === 'Session Review'"
          @gn-click="select('Session Review')">
              <font-awesome-icon icon="fa-solid fa-calendar" />
        </GnTabNavigationItem>

        <GnTabNavigationItem
          label="Decks"
          :selected="activeTab === 'Decks'"
          @gn-click="select('Decks')">
              <font-awesome-icon icon="fa-solid fa-list-check" />
        </GnTabNavigationItem>
       
        <GnTabNavigationItem
          label="Statistics"
          :selected="activeTab === 'Statistics'"
          @gn-click="select('Statistics')">
              <font-awesome-icon icon="fa-solid fa-chart-bar" />
        </GnTabNavigationItem>
        
    </GnTabNavigationBar>

    <SessionDecks v-if="activeTab === 'Decks'" :sessionId="sessionId" />
    <SessionReview v-if="activeTab === 'Session Review'" :sessionId="sessionId" />
    <SessionStatistics v-if="activeTab === 'Statistics'" :sessionId="sessionId"/>
  </div>
</template>

<script setup>
import { ref } from "vue";
import { useRouter, useRoute } from "vue-router";
import { GnTabNavigationBar, GnTabNavigationItem, GnBackBar } from "@graphyne/web-components";
import SessionDecks from './SessionDecks.vue';
import SessionReview from './SessionReview.vue';
import SessionStatistics from './SessionStatistics.vue';

const route = useRoute();
const router = useRouter();
const activeTab = ref('Decks');
const sessionId = route.params.id;

function goBack() {
  router.go(-1);
}

</script>
