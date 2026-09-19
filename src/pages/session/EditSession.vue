<template>
 <div class="flex flex-col">

    <GnBackBar 
        title="Create Deck"
        subtitle="Last updated 04/02/2026"
        textColor="var(--app-color-text)"
        color="var(--app-color-accent)"
        backgroundColor="var(--app-color-surface)"
        borderColor="var(--app-color-surface)"
        @gn-back="goBack">
        <font-awesome-icon icon="fa-solid fa-arrow-left" />  
    </GnBackBar>

    <FormHeader
      header="Edit study session"
      description="Edit attributes for existing study session."
    />

    <EditSessionForm
      v-if="session"
      :sessionId="session.id"
      :sessionName="session.name"
      :sessionDescription="session.description"
    />
  </div>
</template>


<script setup>
import { ref } from "vue";
import { onMounted } from "vue";
import { useRoute, useRouter } from "vue-router";
import { sessionStore } from "../../api";
import EditSessionForm from '../../components/session/EditSessionForm.vue';
import FormHeader from '../../components/shared/forms/FormHeader.vue';

const router = useRouter();
const route = useRoute(); 
const sessionId = route.params.id;
const session = ref(null); 

function goBack() {
  router.go(-1);
}

onMounted(async () => {
  session.value = await sessionStore.viewSession(sessionId);
});
</script>
