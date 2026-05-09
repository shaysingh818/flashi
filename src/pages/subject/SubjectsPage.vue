<template>
  <div class="flex col-center w-100">

  <br /> 
   <SearchComponent 
      text="Search Subjects" 
      @search="search"
    />

    <div class="flex centered-all">
      <h2 style="color: white;">Subjects </h2> 
    </div>

    <SubjectListView />
    
    <HoverButton icon="fa-solid fa-plus" :onClickHandler="createSubject" />

  </div>
  
</template>


<script setup>
import { ref } from 'vue';
import { useRouter } from 'vue-router';
import SearchComponent from '../../components/shared/forms/SearchComponent.vue';
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


