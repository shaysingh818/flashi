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
      header="Edit subject"
      description="Edit Attributes for subject"
    />

    <EditSubjectForm
      v-if="subject"
      :subjectId="subject.id"
      :subjectName="subject.name"
      :subjectDescription="subject.description"
    />
  </div>
</template>

<script setup>
import { ref } from 'vue';
import { onMounted } from 'vue';
import { useRoute } from 'vue-router';
import { subjectStore } from '../../api/Subject';
import FormHeader from '../../components/shared/forms/FormHeader.vue';
import EditSubjectForm from '../../components/subject/EditSubjectForm.vue';


const route = useRoute(); 
const subjectId = route.params.id;
const subject = ref(null); 

onMounted(async () => {
  subject.value = await subjectStore.viewSubject(subjectId);
});

</script>
