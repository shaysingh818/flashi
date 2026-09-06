<template>
  <div class="flex col-center w-100">

  <br /> 

  <div class="w-85">
      <GnSearchFormField 
        v-model="searchTerm"
        inputBorderColor="rgb(28 25 23)" 
        buttonBorderColor="#F97316"
        buttonBackgroundColor="#F97316" 
        color="white" 
        label="Search Subjects" 
        placeholder="Search Subjects"
        backgroundColor="rgb(28 25 23)"
        @gn-search="search"
      />
  </div>

  <br></br>


    <div class="flex centered-all">
      <h2 style="color: white;">Subjects </h2> 
    </div>

    <SubjectListView />
    
    <HoverButton icon="fa-solid fa-plus" @click="createSubject" />

  </div>
  
</template>


<script setup>
import { ref } from 'vue';
import { useRouter } from 'vue-router';
import { GnSearchFormField } from '@graphyne/web-components';
import HoverButton from '../../components/shared/buttons/HoverButton.vue';
import SubjectListView from '../../components/subject/SubjectListView.vue';
import { subjectStore } from '../../api/Subject';

const router = useRouter();
const searchTerm = ref('');

async function search(searchTerm) {
  await subjectStore.searchSubjects(searchTerm);
}

async function createSubject() {
  router.push({ path: "/subjects/create" });
}
</script>


