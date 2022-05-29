<template>
    <div id="frame">
        <router-view :key="$route.fullPath"></router-view>
    </div>
</template>