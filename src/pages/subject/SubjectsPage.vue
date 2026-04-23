<template>
  <div class="page-container">

   <SearchComponent 
      text="Search Subjects" 
      :onClick="search"
    />

    <div class="subjects-header">

      <h2>Subjects </h2> 
    
    </div>

    <SubjectListView />
    
    <HoverButton icon="fa-solid fa-plus" :onClickHandler="createSubject" />

  </div>

</template>

<script>
import SearchComponent from '../../components/shared/SearchComponent.vue'
import HoverButton from '../../components/shared/HoverButton.vue'
import SubjectListView from '../../components/subject/SubjectListView.vue'

export default {
  components: { 
    SearchComponent, 
    SubjectListView,
    HoverButton
  },
  methods: {
    async search() {
      console.log("test"); 
    },
    async createSubject() {
      this.$router.push({ path: "/subjects/create"})
    }
  },
}
</script>

<style scoped>

h2 {
  color: white; 
}


.subjects-header {
  display: flex; 
  align-items: center; 
  justify-content: center; 
  width: 100%; 
}

.subjects-list-view {
  display: flex; 
  flex-direction: column; 
  align-items: center; 
  justify-content: center; 
  gap: 0.5rem; 
}

</style>
