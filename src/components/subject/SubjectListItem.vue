<template>

  <GnListTile 
    :description="description" 
    :title="title"
    borderColor="var(--app-color-background)"
    textColor="var(--app-color-text)"
    descriptionColor="var(--app-color-text-muted)"
    @gn-click="handleClick">
        
    <template #leading>
        <font-awesome-icon icon="fa-regular fa-calendar" />
    </template> 
    
    <template #trailing>

      <GnDropDownMenu
        backgroundColor="var(--app-color-surface)"
        borderColor="var(--app-color-surface)"
        align="right"
        :disabled="false"
        closeOnEscape="true"
        closeOnOutsideClick="true">
          <template #trigger>
            <GnIconButton label="Options" variant="ghost">
              <font-awesome-icon icon="fa-solid fa-ellipsis-vertical" />
            </GnIconButton>
          </template>
          <template #items>
            <GnDropDownMenuItem label="Edit Subject" @gn-click="editSubject ">
              <font-awesome-icon icon="fa-solid fa-pen-to-square" />
            </GnDropDownMenuItem>
            <GnDropDownMenuItem label="Delete Subject" color="#dc2626" text-color="#dc2626" @gn-click="deleteSubject">
              <font-awesome-icon icon="fa-solid fa-x" />
            </GnDropDownMenuItem>
          </template>
      </GnDropDownMenu>
    </template>
  </GnListTile>

</template>

<script setup>
import { ref } from "vue";
import { useRouter } from "vue-router";
import { message } from '@tauri-apps/plugin-dialog';
import { GnListTile  } from "@graphyne/web-components";
import { subjectStore } from "../../api/Subject";

const props = defineProps({
  id: {
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
  dateUpdated: {
    type: String,
    required: true,
  },
  actionType: {
    type: String,
    required: false,
    default: "view",
  },
  sessionId: {
    type: String,
    required: false,
  },
});

const router = useRouter();
const isDropdownOpen = ref(false);
const showDialog = ref(false);

function showDropdown() {
  isDropdownOpen.value = !isDropdownOpen.value;
}

async function deleteSubject() {
  const deleteResult = await subjectStore.deleteSubject(`${props.id}`);
  if (deleteResult === "Success") {    
    await message(
      'Successfully deleted subject',
      { title: 'Recalli', kind: 'info' }
    );
    await subjectStore.loadSubjects();
  } else {    
    await message(
      'Could not delete subject',
      { title: 'Recalli', kind: 'info' }
    );
  }
}

function editSubject() {
  router.push({ path: `/subjects/edit/${props.id}` });
}

function subjectDetail() {
  router.push({ path: `/subjects/detail/${props.id}` });
}

function deckAddDetail() {
  router.push({ path: `/sessions/deck/add/${props.sessionId}` });
}

function handleClick() {
  if (props.actionType === "add") {
    deckAddDetail();
  } else {
    subjectDetail();
  }
}
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
