<template>
  <div id="app">
    <SideMenu/>
  </div>
</template>

<script setup>
import SideMenu from './components/SideMenu.vue';
</script>

<style>
html, body, #app {
  height: 100%;
  margin: 0;
  padding: 0;
  overflow: hidden; /* 隐藏滚动条 */
}

#app {
  display: flex;
}
</style>