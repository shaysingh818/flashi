<template>
  <div class="deck-list-item">
    <div class="container row centered g-1" @click="deckDetail">
      <div class="container centered align-center leading">
        <font-awesome-icon
          icon="fa-solid fa-list-check"
          class="list-item-icon-style"
        />
      </div>

      <div class="container col left content">
        <h1>{{ title }}</h1>
        <p>{{ description }}</p>
        <div class="container left row g-1 p-5" style="height: 20px">
          <Tag v-for="(item, index) in tags" :key="index" :title="item" />
        </div>
      </div>

      <div class="container trailing centered align-center g-1">
        <p :style="{ color: '#F97316' }">{{ dateUpdated }}</p>
        <font-awesome-icon
          @click.stop="showDropdown"
          icon="fa-solid fa-ellipsis-vertical"
          class="trailing-icon-style"
        />

        <div v-if="isDropdownOpen" class="dropdown-content">
          <DropDownItem
            icon="fa-solid fa-pen-to-square"
            text="Edit Deck"
            :onClickHandler="editDeck"
          />
          <DropDownItem
            icon="fa-solid fa-x"
            text="Delete Deck"
            :onClickHandler="deleteDeck"
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
  </div>
</template>

<script>
import { svgPathData } from "@fortawesome/free-solid-svg-icons/faRotateRight";
import ConfirmDialog from "../shared/ConfirmDialog.vue";
import Tag from "../shared/Tag.vue";
import DropDownItem from "../shared/DropDownItem.vue";

export default {
  name: "DeckListItem",
  components: { ConfirmDialog, Tag, DropDownItem },
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
    deleteDeck() {
      this.isDropdownOpen = false; // close the dropdown too
      this.showDialog = true;
    },
    editDeck() {
      this.$router.push({ path: "/deck/edit" });
    },
    deckDetail() {
      this.$router.push({ path: "/deck/detail" });
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

.list-item-trailing svg:hover {
  background-color: gray;
}

.container.leading svg {
  width: 35px;
  height: 35px;
  color: #f97316;
}

.trailing-icon-style {
  color: #b5b3b3;
}

.dropdown-content {
  position: absolute;
  display: flex;
  flex-direction: column;
  border-radius: 4px;
  padding: 10px;
  gap: 0.5rem;
  background-color: rgb(28 25 23);
  transform: translateX(25%) translateY(60%);
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
