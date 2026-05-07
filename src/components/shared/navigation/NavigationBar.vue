<template>

  <div class="navigation">

    <div class="navigation-logo">
      <Logo /> 
    </div>

    <div class="navigation-items">

      <RouterLink to="/" >
        <NavigationItem icon="house" navTitle="home" />
      </RouterLink>

      <RouterLink to="/subjects">
        <NavigationItem icon="clone" navTitle="subjects" />
      </RouterLink>

      <RouterLink to="/sessions">
        <NavigationItem icon="calendar" navTitle="sessions" />
      </RouterLink>

      <RouterLink to="/metrics">
        <NavigationItem icon="chart-bar" navTitle="metrics" />
      </RouterLink>

      <RouterLink to="/settings">
        <NavigationItem icon="sun" navTitle="settings" />
      </RouterLink>

    </div> 
    
  </div> 

</template>

<script setup>
import Logo from '../../../assets/flashi.svg?component'
import NavigationItem from './NavigationItem.vue'
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
  height: 15%;
  display: flex;
  justify-content: center;
  align-items: center;
  padding: 2px;
}

.navigation-items {
  display: flex; 
  flex-direction: column;
  padding: 10px;
  height: 75%;
  gap: 1.0rem;
  align-items: center;
}


.navigation-logo svg {
  width: 100%; 
  height: auto;
}

a {
  text-decoration: none; 
}
</style> 
