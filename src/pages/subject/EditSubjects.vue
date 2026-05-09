<template>
  <div class="flex col-center w-100">

    <br></br>
    
    <FormHeader
      header="Edit subject"
      description="Edit Attributes for subject"
    />
    <br />

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
