<template>
  <div class="flex row w-100 list-item" style="height: 50px;" @click="subjectDetail">
    <div class="flex leading centered-all">
      <font-awesome-icon
        icon="fa-regular fa-clone"
        class="list-item-icon-style"
      />
    </div>

    <div class="content">
      <h1>{{ title }}</h1>
      <p>{{ description }}</p>
    </div>

    <div class="flex row gap-4 trailing centered-all">
      <p :style="{ color: '#F97316' }">{{ dateUpdated }}</p>
      <font-awesome-icon
        @click.stop="showDropdown"
        icon="fa-solid fa-ellipsis-vertical"
        class="trailing-icon-style"
      />

      <div v-if="isDropdownOpen" class="flex col absolute gap-2 p-2 transform-50-80 rounded-1" style="background-color: rgb(28 25 23);">

        <DropDownItem
          icon="fa-solid fa-pen-to-square"
          text="Edit Subject"
          :onClickHandler="editSubject"
        />
        <DropDownItem
          icon="fa-solid fa-x"
          text="Delete Subject"
          :onClickHandler="deleteSubject"
        />
        
      </div>
      <Teleport to="body">
        <ConfirmDialog
          v-if="showDialog"
          text="Are you sure you want to delete the subject"
          @close="showDialog = false"
          @confirm="showDialog = false"
        />
      </Teleport>
    </div>
  </div>
</template>

<script>
import { svgPathData } from "@fortawesome/free-solid-svg-icons/faRotateRight";
import ConfirmDialog from "../shared/ConfirmDialog.vue";
import DropDownItem from "../shared/navigation/DropDownItem.vue";

export default {
  name: "SubjectListItem",
  components: { ConfirmDialog, DropDownItem },
  props: {
    title: {
      type: String,
      required: true,
    },
    description: {
      type: String,
      required: true,
    },
    dateUpdated: {
      type: String,
      required: true,
    },
  },
  data() {
    return {
      isDropdownOpen: false,
      showDialog: false,
    };
  },
  methods: {
    showDropdown() {
      console.log("Show dropdown", this.isDropdownOpen);
      this.isDropdownOpen = !this.isDropdownOpen;
    },
    deleteSubject() {
      this.isDropdownOpen = false; // close the dropdown too
      this.showDialog = true;
    },
    editSubject() {
      this.$router.push({ path: "/subjects/edit" });
    },
    subjectDetail() {
      this.$router.push({ path: "/subjects/detail" });
    },
  },
};
</script>

<style scoped>

.list-item:hover {
  background-color: rgb(28 25 23);
}

.list-item-icon-style {
  color: #f97316;
  width: 25px; 
  height: 25px;
}


.trailing-icon-style {
  color: #b5b3b3;
}

.dropdown-icon-style {
  color: #f97316;
}

h1 {
  font-size: 15px;
  margin: 0;
}

p {
  color: #b5b3b3;
  font-size: 10px;
  margin: 0;
}
</style>
