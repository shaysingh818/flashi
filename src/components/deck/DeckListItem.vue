<template>
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

      <GnDropDownMenu
        backgroundColor="var(--app-color-surface)"
        borderColor="var(--app-color-surface)"
        align="right"
        :disabled="false"
        closeOnEscape="true"
        closeOnOutsideClick="true"
        v-if="props.actionType === 'view'">
          <template #trigger>
            <GnIconButton label="Options" variant="ghost">
              <font-awesome-icon icon="fa-solid fa-ellipsis-vertical" />
            </GnIconButton>
          </template>
          <template #items>
            <GnDropDownMenuItem label="Edit Deck" @gn-click="editDeck">
              <font-awesome-icon icon="fa-solid fa-pen-to-square" />
            </GnDropDownMenuItem>
            <GnDropDownMenuItem label="Delete Deck" color="#dc2626" text-color="#dc2626" @gn-click="deleteDeck">
              <font-awesome-icon icon="fa-solid fa-x" />
            </GnDropDownMenuItem>
          </template>
      </GnDropDownMenu>

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
