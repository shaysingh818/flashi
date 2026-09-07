<template>
  <div class="flex flex-col">

    <div class="p-4">      
        <GnSearchFormField 
            v-model="searchTerm"
            inputBorderColor="rgb(28 25 23)" 
            buttonBorderColor="var(--app-color-accent)"
            buttonBackgroundColor="var(--app-color-accent)" 
            buttonIconColor="var(--app-color-text)"
            focusColor="var(--app-color-accent)" 
            textColor="var(--app-color-accent)"
            label="Search Sessions" 
            placeholder="Search Sessions"
            backgroundColor="var(--app-color-surface)"
            @gn-search="search"
        />
    </div>

    <div class="flex items-center justify-center">
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


