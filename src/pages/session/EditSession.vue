<template>
 <div class="flex col-center w-100 g-1">
   <br></br>

    <FormHeader
      header="Edit study session"
      description="Edit attributes for existing study session."
    />
    <br />

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
import { useRoute } from "vue-router";
import { sessionStore } from "../../api";
import EditSessionForm from '../../components/session/EditSessionForm.vue';
import FormHeader from '../../components/shared/forms/FormHeader.vue';

const route = useRoute(); 
const sessionId = route.params.id;
const session = ref(null); 

onMounted(async () => {
  session.value = await sessionStore.viewSession(sessionId);
});
</script>
