<template>

<div class="dropdown-content">
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

</template>

<script>

</script>

<style scoped>

</style>