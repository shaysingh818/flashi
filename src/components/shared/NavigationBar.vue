<template>

  <div class="navigation">

    <div class="navigation-logo">
      <Logo /> 
    </div>

    <div class="navigation-items">

    </div> 
    
  </div> 

</template>

<script setup>
import Logo from '../../assets/flashi.svg?component'
</script>

<style scoped>

.navigation {
  display: flex; 
  flex-direction: column;
  gap: 1.0rem; 
  width: 55px;
  height: 100vh;
  position: fixed;
  background-color: rgb(28 25 23);
  padding: 5px; 
}

.navigation-logo {
  height: auto;
  display: flex;
  justify-content: center;
  align-items: center;
  padding: 2px; 
}

.navigation-items {
  display: flex; 
  flex-direction: column;
  padding: 10px;
  height: auto; 
}


.navigation-logo svg {
  width: 100%; 
  height: auto;
}
</style> 
