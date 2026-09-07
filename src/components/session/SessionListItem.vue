<template>
  <!-- <div class="flex row w-100 list-item" style="height: 50px;" @click="sessionDetail">
    <div class="flex leading centered-all">
      <font-awesome-icon
        icon="fa-regular fa-calendar"
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

      <div 
        v-if="isDropdownOpen" 
        class="flex col absolute gap-2 p-2 transform-50-80 rounded-1" 
        style="background-color: rgb(28 25 23);"
      >
        <DropDownItem
          icon="fa-solid fa-pen-to-square"
          text="Edit Session"
          :onClickHandler="editSession"
        />
        <DropDownItem
          icon="fa-solid fa-x"
          text="Delete Session"
          :onClickHandler="deleteSession"
        />
      </div>

      <Teleport to="body">
        <ConfirmDialog
          v-if="showDialog"
          text="Deleting a session will delete all the metrics and session review questions associated with it. Are you sure you want to delete this session?"
          @close="showDialog = false"
          @confirm="confirmDelete"
        />
      </Teleport>
    </div>
  </div> -->

  <GnListTile 
    :description="description" 
    :title="title"
    borderColor="var(--app-color-background)"
    textColor="var(--app-color-text)"
    descriptionColor="var(--app-color-text-muted)"
    @gn-click="sessionDetail">
        
    <template #leading>
        <font-awesome-icon icon="fa-regular fa-calendar" />
    </template> 
    
    <template #trailing>
       <GnIconButton label="More options" variant="ghost" size="sm">
           <font-awesome-icon
             @click.stop="showDropdown"
             icon="fa-solid fa-ellipsis-vertical"
           />
       </GnIconButton>
    </template>
  </GnListTile>


</template>

<script setup>
import { ref } from "vue";
import { sessionStore } from  "../../api"; 
import { useRouter } from "vue-router";
import { svgPathData } from "@fortawesome/free-solid-svg-icons/faRotateRight";
import { GnListTile } from "@graphyne/web-components";
import ConfirmDialog from "../shared/ConfirmDialog.vue";
import DropDownItem from "../shared/navigation/DropDownItem.vue";

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
});

const router = useRouter();
const showDialog = ref(false);
const isDropdownOpen = ref(false);

function showDropdown() {
  isDropdownOpen.value = !isDropdownOpen.value;
}

async function confirmDelete() {
  const result = await sessionStore.deleteSession(`${props.id}`);
  await sessionStore.loadSessions();
  showDialog.value = false;
}

function deleteSession() {
  isDropdownOpen.value = false;
  showDialog.value = true;
}

function editSession() {
  router.push({ path: `/sessions/edit/${props.id}` });
}

function sessionDetail() {
  router.push({ path: `/sessions/detail/${props.id}` });
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
