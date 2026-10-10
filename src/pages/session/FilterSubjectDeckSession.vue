<template>
  <div class="flex flex-col">
    <GnBackBar 
        title="Subjects within application"
        subtitle="Last updated 04/02/2026"
        textColor="var(--app-color-text)"
        color="var(--app-color-accent)"
        backgroundColor="var(--app-color-surface)"
        borderColor="var(--app-color-surface)"
        @gn-back="goBack">
        <font-awesome-icon icon="fa-solid fa-arrow-left" />  
    </GnBackBar>

    <div class="p-6">
      <GnSearchFormField 
        v-model="searchTerm"
        inputBorderColor="rgb(28 25 23)" 
        buttonBorderColor="var(--app-color-accent)"
        buttonBackgroundColor="var(--app-color-accent)" 
        buttonIconColor="var(--app-color-text)"
        focusColor="var(--app-color-accent)" 
        textColor="var(--app-color-accent)"
        label="Search subjects in application" 
        placeholder="Search subjects in application"
        backgroundColor="var(--app-color-surface)"
        @gn-search="search"
      />
    </div>

    <SubjectListView
        :subjects="subjectStore.subjects"
        :sessionId="sessionId"
        actionType="add"
    />

  </div>
</template>

<script setup>
import { onMounted } from "vue";
import { useRoute } from "vue-router";
import { useRouter } from "vue-router";
import { subjectStore } from "../../api";
import SubjectListView from '../../components/subject/SubjectListView.vue';

const router = useRouter(); 
const route = useRoute(); 
const sessionId = route.params.id;

onMounted(async () => {
  await subjectStore.loadSubjects();
});

async function search(searchTerm) {
  await subjectStore.searchSubjects(searchTerm);
}

function goBack() {
  router.go(-1);
}

</script>

