<template>
  <!-- <div class="flex row w-100 list-item">
    <div class="flex row-center w-100" @click="deckDetail">
      <div class="flex leading centered-all">
        <font-awesome-icon
          icon="fa-solid fa-list-check"
          class="list-item-icon-style"
        />
      </div>

      <div class="content">
        <h1>{{ title }}</h1>
        <p>{{ description }}</p>
        <div class="flex items-left row gap-2 p-2" style="height: 20px">
          <Tag v-for="(item, index) in tags" :key="index" :title="item" />
        </div>
      </div>

      <div class="flex row gap-4 trailing centered-all">
        <p :style="{ color: '#F97316' }">{{ dateUpdated }}</p>
        <font-awesome-icon
          v-if="props.actionType === 'view'"
          @click.stop="showDropdown"
          icon="fa-solid fa-ellipsis-vertical"
          class="trailing-icon-style"
        />

        <font-awesome-icon
          v-if="props.actionType === 'add'"
          @click.stop="addDeckToSession"
          icon="fa-solid fa-plus"
          class="trailing-icon-style"
        />

        <font-awesome-icon
          v-if="props.actionType === 'remove'"
          @click.stop="removeDeckFromSession"
          icon="fa-solid fa-x"
          class="trailing-icon-style"
        />

        <div
          v-if="isDropdownOpen"
          class="flex col absolute gap-2 p-2 transform-50-80 rounded-1"
          style="background-color: rgb(28 25 23)"
        >
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
            @confirm="confirmDelete"
          />
        </Teleport>
      </div>
    </div>
  </div> -->
  <GnListTile 
    :description="description" 
    :title="title"
    borderColor="var(--app-color-background)"
    textColor="var(--app-color-text)"
    descriptionColor="var(--app-color-text-muted)"
    @gn-click="deckDetail">
        
    <template #leading>
        <font-awesome-icon icon="fa-solid fa-list-check" />
    </template> 

    <template #tags>
        <GnTag v-for="(item, index) in tags" :key="index" :label="item" />
    </template>
    
    <template #trailing>
       <GnIconButton  @gn-click="showDropdown" label="More options" variant="ghost" size="sm" v-if="props.actionType === 'view'">
          <font-awesome-icon icon="fa-solid fa-ellipsis-vertical" />
       </GnIconButton>

       <GnIconButton @gn-click="addDeckToSession" label="More options" variant="ghost" size="sm" v-if="props.actionType === 'add'">
          <font-awesome-icon icon="fa-solid fa-plus" />
       </GnIconButton>

       <GnIconButton @gn-click="removeDeckFromSession" label="More options" variant="ghost" size="sm" v-if="props.actionType === 'remove'">
          <font-awesome-icon icon="fa-solid fa-x" />
       </GnIconButton>
    </template>
  </GnListTile>
  
</template>

<script setup>
import { ref } from "vue";
import { useRouter } from "vue-router";
import { svgPathData } from "@fortawesome/free-solid-svg-icons/faRotateRight";
import { deckStore } from "../../api/Deck";
import ConfirmDialog from "../shared/ConfirmDialog.vue";
import DropDownItem from "../shared/navigation/DropDownItem.vue";
import { GnTag, GnListTile } from "@graphyne/web-components";


const props = defineProps({  
  id: {
    type: String,
    required: true,
  },  
  subjectId: {
    type: String,
    required: true,
  },
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
  actionType: {
    type: String,
    required: false,
    default: "view",
  },
});

const router = useRouter();
const isDropdownOpen = ref(false);
const showDialog = ref(false);
const emit = defineEmits(["deck:add_to_session", "deck:remove_from_session"]);

function showDropdown() {
  isDropdownOpen.value = !isDropdownOpen.value;
}

async function confirmDelete() {
  const result = await deckStore.deleteDeck(`${props.id}`);
  console.log(result);
  await deckStore.loadDecks(`${props.subjectId}`);
  showDialog.value = false;
}

async function deleteDeck() {
  isDropdownOpen.value = false;
  showDialog.value = true;
}

function editDeck() {
  router.push({ path: `/deck/edit/${props.id}` });
}

function deckDetail() {
  router.push({ path: `/deck/detail/${props.id}` });
}

function addDeckToSession() {
  emit("deck:add_to_session", props.id); 
}

function removeDeckFromSession() {
  emit("deck:remove_from_session", props.id); 
}

</script>

<style scoped>
.deck-list-item {
  height: 75px;
  width: 100%;
}

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
