<template>
  <div class="subject-list-item" @click="subjectDetail">
    <div class="list-item-icon">
      <font-awesome-icon
        icon="fa-regular fa-clone"
        class="list-item-icon-style"
      />
    </div>

    <div class="content">
      <h1>{{ title }}</h1>
      <p>{{ description }}</p>
    </div>

    <div class="trailing-content">
      <p :style="{ color: '#F97316' }">{{ dateUpdated }}</p>
      <font-awesome-icon
        @click.stop="showDropdown"
        icon="fa-solid fa-ellipsis-vertical"
        class="trailing-icon-style"
      />

      <div v-if="isDropdownOpen" class="dropdown-content">
        <div @click.stop="editSubject" class="dropdown-item">
          <font-awesome-icon
            icon="fa-solid fa-pen-to-square"
            class="dropdown-icon-style"
          />
          <p>Edit Subject</p>
        </div>

        <div @click.stop="deleteSubject" class="dropdown-item">
          <font-awesome-icon icon="fa-solid fa-x" class="dropdown-icon-style" />
          <p>Delete Subject</p>
        </div>
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

export default {
  name: "SubjectListItem",
  components: { ConfirmDialog },
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
.subject-list-item {
  display: flex;
  flex-direction: row;
  width: 100%;
  height: 50px;
}

.subject-list-item:hover {
  background-color: rgb(28 25 23);
}

.list-item-icon {
  flex: none;
  width: 50px;
  height: 50px;
}

.trailing-content svg:hover {
  background-color: gray;
}

.list-item-icon svg {
  width: 80%;
  height: auto;
}

.list-item-icon-style {
  color: #f97316;
}

.trailing-icon-style {
  color: #b5b3b3;
}

.dropdown-icon-style {
  color: #f97316;
}

.content {
  display: flex;
  flex-direction: column;
  justify-content: left;
  white-space: nowrap;
  overflow: hidden;
  width: 60%;
  padding: 5px;
}

.trailing-content {
  display: flex;
  flex-direction: row;
  width: 30%;
  align-items: center;
  justify-content: center;
  gap: 1rem;
}

.dropdown-content {
  position: absolute;
  display: flex;
  flex-direction: column;
  border-radius: 4px;
  padding: 10px;
  gap: 0.5rem;
  background-color: rgb(28 25 23);
  transform: translateX(50%) translateY(80%);
}

.dropdown-item {
  display: flex;
  flex-direction: row;
  justify-content: left;
  align-items: left;
  gap: 0.5rem;
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
