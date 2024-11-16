<template>
  <div ref="menuContainer" tabindex="0" style="outline: none;">
    <el-container style="height: 100vh;">
      <el-row>
        <el-col :span="6" style="border-right: 1px solid #dcdfe6;">
          <el-menu
              :default-active="selectedItem"
              @select="handleSelect"
              style="height: 100vh; overflow-y: auto;"
          >
            <el-menu-item
                v-for="menuItem in items"
                :key="menuItem.index"
                :index="menuItem.index"
                :class="{ 'active-menu-item': selectedItem === menuItem.index }"
            >
              {{ menuItem.name }}
            </el-menu-item>
          </el-menu>
        </el-col>
        <el-col :span="18" style="padding: 20px;">
          <el-card v-if="currentItem" class="box-card">
            <div slot="header">
              <span>{{ currentItem.name }} Details</span>
            </div>
            <div>
              Details for {{ currentItem.name }} go here.
            </div>
          </el-card>
        </el-col>
      </el-row>
    </el-container>
  </div>
</template>

<script setup lang="ts">
import {ref, onMounted, computed, nextTick} from 'vue';
import { invoke } from "@tauri-apps/api/core";

const items = ref([
  {index: '1', name: 'Item 1'},
  {index: '2', name: 'Item 2'},
  {index: '3', name: 'Item 3'},
  {index: '4', name: 'Item 4'},
  {index: '5', name: 'Item 5'},
  {index: '6', name: 'Item 6'},
  {index: '7', name: 'Item 7'},
  {index: '8', name: 'Item 8'},
  {index: '9', name: 'Item 9'},
  {index: '10', name: 'Item 10'},
  {index: '11', name: 'Item 11'},
  {index: '12', name: 'Item 12'},
]);

const selectedItem = ref('1');

const currentItem = computed(() => {
  return items.value.find(item => item.index === selectedItem.value);
});

function handleSelect(key) {
  selectedItem.value = key;
  nextTick(() => {
    scrollToSelected();
  });
}

function scrollToSelected() {
  const menu = document.querySelector('.el-menu');
  const activeItem = menu.querySelector('.active-menu-item');
  if (activeItem) {
    const menuScrollTop = menu.scrollTop;
    const menuOffsetHeight = menu.offsetHeight;
    const itemOffsetTop = activeItem.offsetTop;
    const itemOffsetHeight = activeItem.offsetHeight;

    if (itemOffsetTop < menuScrollTop) {
      menu.scrollTop = itemOffsetTop;
    } else if (itemOffsetTop + itemOffsetHeight > menuScrollTop + menuOffsetHeight) {
      menu.scrollTop = itemOffsetTop - menuOffsetHeight + itemOffsetHeight;
    }
  }
}

function handleKeydown(event) {
  const itemKeys = items.value.map(item => item.index);
  let index = itemKeys.indexOf(selectedItem.value);

  if (event.key === 'ArrowDown') {
    index = (index + 1) % itemKeys.length;
  } else if (event.key === 'ArrowUp') {
    index = (index - 1 + itemKeys.length) % itemKeys.length;
  }

  selectedItem.value = itemKeys[index];

  nextTick(() => {
    scrollToSelected();
  });
}

const menuContainer = ref(null);

onMounted(() => {
  invoke('my_custom_command');
  nextTick(() => {
    if (menuContainer.value) {
      menuContainer.value.focus(); // 聚焦菜单容器
    }
    window.addEventListener('keydown', handleKeydown);
  });
});
</script>

<style scoped>
.el-container {
  display: flex;
  height: 100vh;
  margin: 0;
  padding: 0;
}

.el-menu-item {
  padding-left: 10px;
}

.active-menu-item {
  background-color: #add8e6 !important; /* 浅蓝色 */
  color: #303133 !important;
  border-radius: 8px; /* 设置圆角 */
  margin: 2px 0;
}

.el-card {
  margin-bottom: 20px;
}

.el-main {
  flex: 1;
  padding: 20px;
}

.box-card {
  border: none;
  box-shadow: 0 2px 12px 0 rgba(0, 0, 0, 0.1);
}
</style>