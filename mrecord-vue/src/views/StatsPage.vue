<script setup lang="ts">
</script>

<template>
  <div class="placeholder-page">
    <div class="page-header">
      <h2>统计</h2>
    </div>
    <div class="page-body">
      <div class="empty-block" style="height: 200px;"></div>
      <div class="empty-block" style="height: 160px;"></div>
      <div class="empty-block" style="height: 120px;"></div>
    </div>
  </div>
</template>

<style scoped>
.placeholder-page {
  min-height: 100vh;
  background: #f5f5f5;
}
.page-header {
  background: #fff;
  padding: 16px;
  padding-top: calc(16px + env(safe-area-inset-top, 0px));
}
.page-header h2 {
  font-size: 18px;
  font-weight: 600;
  color: #333;
}
.page-body {
  padding: 16px;
  display: flex;
  flex-direction: column;
  gap: 12px;
}
.empty-block {
  background: #fff;
  border-radius: 12px;
  box-shadow: 0 1px 4px rgba(0,0,0,0.03);
}
</style>
