<template>
  <div class="deck-list-item">
    <div class="list-item" @click="subjectDetail">
      <div class="list-item-leading">
        <font-awesome-icon
          icon="fa-solid fa-list-check"
          class="list-item-icon-style"
        />
      </div>

      <div class="list-item-content">
        <h1>{{ title }}</h1>
        <p>{{ description }}</p>
        <div class="tag-section">
          <Tag v-for="(item, index) in tags" :key="index" :title="item" />
        </div>
      </div>

      <div class="list-item-trailing">
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
            <font-awesome-icon
              icon="fa-solid fa-x"
              class="dropdown-icon-style"
            />
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
  </div>
</template>

<script>
import { svgPathData } from "@fortawesome/free-solid-svg-icons/faRotateRight";
import ConfirmDialog from "../shared/ConfirmDialog.vue";
import Tag from "../shared/Tag.vue";

export default {
  name: "DeckListItem",
  components: { ConfirmDialog, Tag },
  props: {
    title: {
      type: String,
      required: true,
    },
    description: {
      type: String,
      required: true,
    },
    tags: {
      type: Array,
      required: false,
      default: () => [],
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
.deck-list-item {
  height: 75px;
  width: 100%;
}

.list-item:hover {
  background-color: rgb(28 25 23);
}

.trailing-content svg:hover {
  background-color: gray;
}

.list-item-leading svg {
  width: 35px;
  height: 35px;
  color: #f97316;
}

.trailing-icon-style {
  color: #b5b3b3;
}

.dropdown-icon-style {
  color: #f97316;
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

.tag-section {
  display: flex;
  flex-direction: row;
  gap: 0.5rem;
  height: 20px;
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
