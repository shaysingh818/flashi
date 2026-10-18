<script setup lang="ts">
import SiteHeader from "./components/SiteHeader.vue";
import HeroSection from "./components/HeroSection.vue";
import FeaturesSection from "./components/FeaturesSection.vue";
import QuestionTypesSection from "./components/QuestionTypesSection.vue";
import DownloadSection from "./components/DownloadSection.vue";
import SiteFooter from "./components/SiteFooter.vue";
</script>

<template>
  <SiteHeader />
  <main>
    <HeroSection />
    <FeaturesSection />
    <QuestionTypesSection />
    <DownloadSection />
  </main>
  <SiteFooter />
</template>
