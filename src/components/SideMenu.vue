<template>
    <div ref="menuContainer" tabindex="0" style="height: 100vh; width: 100vw">
        <el-container style="height: 100vh">
            <el-row>
                <el-col :span="6" style="border-right: 1px solid #dcdfe6">
                    <el-menu
                        :default-active="selectedItem"
                        @select="handleSelect"
                        style="height: 100vh; overflow-y: auto"
                    >
                        <el-menu-item
                            v-for="menuItem in titleItems"
                            :key="menuItem.index"
                            :index="menuItem.index"
                            :class="{
                                'active-menu-item':
                                    selectedItem === menuItem.index,
                            }"
                        >
                            {{ menuItem.name }}
                        </el-menu-item>
                    </el-menu>
                </el-col>
                <el-col :span="18" style="padding: 20px">
                    <el-container v-if="currentItem" class="box-card">
                        <div slot="header">
                            <span>{{ currentItem.name }}</span>
                        </div>
                    </el-container>
                </el-col>
            </el-row>
        </el-container>
    </div>
</template>

<script setup lang="ts">
import { ref, onMounted, computed, nextTick } from "vue";
import { invoke } from "@tauri-apps/api/core";

interface itemDetailInfo {
    index: number;
    name: string;
}
const titleItems = ref<itemDetailInfo[]>([]);

const detailItems = ref<itemDetailInfo[]>([]);

const selectedItem = ref(1);

const currentItem = computed(() => {
    return detailItems.value.find((item) => item.index === selectedItem.value);
});

function handleSelect(key: number) {
    selectedItem.value = key;
    nextTick(() => {
        scrollToSelected();
    });
}

function scrollToSelected() {
    const menu = document.querySelector(".el-menu");
    if (menu === null) {
        return;
    }
    const activeItem = menu.querySelector(".active-menu-item");
    if (
        menu instanceof HTMLElement &&
        activeItem &&
        activeItem instanceof HTMLElement
    ) {
        const menuScrollTop = menu.scrollTop;
        const menuOffsetHeight = menu.offsetHeight;
        const itemOffsetTop = activeItem.offsetTop;
        const itemOffsetHeight = activeItem.offsetHeight;

        if (itemOffsetTop < menuScrollTop) {
            menu.scrollTop = itemOffsetTop;
        } else if (
            itemOffsetTop + itemOffsetHeight >
            menuScrollTop + menuOffsetHeight
        ) {
            menu.scrollTop =
                itemOffsetTop - menuOffsetHeight + itemOffsetHeight;
        }
    }
}

function handleKeydown(event: any) {
    const itemKeys = titleItems.value.map((item) => item.index);
    let index = itemKeys.indexOf(selectedItem.value);

    if (event.key === "ArrowDown") {
        index = (index + 1) % itemKeys.length;
    } else if (event.key === "ArrowUp") {
        index = (index - 1 + itemKeys.length) % itemKeys.length;
    }

    selectedItem.value = itemKeys[index];

    nextTick(() => {
        scrollToSelected();
    });
}

interface PasteData {
    title_list: string[];
    detail_list: string[];
    size: number;
}

async function fetchPasteData() {
    invoke("get_now_paste").then((data) => {
        let myData = data as PasteData;
        titleItems.value = myData.title_list.map((title, index) => {
            return {
                index: index,
                name: title,
            };
        });
        detailItems.value = myData.detail_list.map((title, index) => {
            return {
                index: index,
                name: title,
            };
        });
    });
}

const menuContainer = ref<HTMLElement | null>(null);

onMounted(() => {
    fetchPasteData();
    console.log("输出到这里");
    nextTick(() => {
        if (menuContainer.value) {
            menuContainer.value.focus(); // 聚焦菜单容器
        }
        window.addEventListener("keydown", handleKeydown);
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
    border-radius: 6px; /* 设置圆角 */
    margin: 2px 0;
}

.box-card {
    box-shadow: 0 2px 12px 0 rgba(0, 0, 0, 0.1);
    overflow-y: auto;
    overflow-x: hidden; /* 隐藏水平滚动 */
    background-color: #a3c0d1;
    color: #333;
    width: 270px;
    line-height: 20px;
}
</style>
