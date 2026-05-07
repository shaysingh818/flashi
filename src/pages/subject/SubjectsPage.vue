<template>
  <div class="flex col-center w-100">

  <br /> 
   <SearchComponent 
      text="Search Subjects" 
      :onClick="search"
    />

    <div class="flex centered-all">
      <h2 style="color: white;">Subjects </h2> 
    </div>

    <SubjectListView />
    
    <HoverButton icon="fa-solid fa-plus" :onClickHandler="createSubject" />

  </div>
  
  

</template>

<script>
import SearchComponent from '../../components/shared/forms/SearchComponent.vue'
import HoverButton from '../../components/shared/buttons/HoverButton.vue'
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



