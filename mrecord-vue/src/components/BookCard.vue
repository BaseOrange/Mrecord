<script setup lang="ts">
import type { BookInfo } from '@/api/modules/book'

defineProps<{
  book: BookInfo
}>()

defineEmits<{
  more: [book: BookInfo]
}>()
</script>

<template>
  <div class="book-card">
    <!-- 3D 立体书本 -->
    <div class="book-3d">
      <div class="spine"></div>
      <div class="pages"></div>
      <div class="cover">
        <svg class="cover-icon" viewBox="0 0 24 24" width="22" height="22">
          <path d="M4 4.5C4 3.12 5.12 2 6.5 2H20v20H6.5C5.12 22 4 20.88 4 19.5V4.5z" fill="none" stroke="currentColor" stroke-width="1.2"/>
          <path d="M4 19.5C4 18.12 5.12 17 6.5 17H20" fill="none" stroke="currentColor" stroke-width="1.2"/>
          <line x1="9" y1="7" x2="15" y2="7" stroke="currentColor" stroke-width="1.2" stroke-linecap="round"/>
          <line x1="9" y1="10" x2="13" y2="10" stroke="currentColor" stroke-width="1.2" stroke-linecap="round"/>
        </svg>
      </div>
    </div>

    <!-- 账簿名称 -->
    <div class="book-name">{{ book.bookName.length > 6 ? book.bookName.slice(0, 6) + '...' : book.bookName }}</div>

    <!-- 更多按钮 -->
    <button class="more-btn" @click.stop="$emit('more', book)">
      <svg viewBox="0 0 24 24" width="18" height="18">
        <circle cx="12" cy="6" r="1.5" fill="currentColor"/>
        <circle cx="12" cy="12" r="1.5" fill="currentColor"/>
        <circle cx="12" cy="18" r="1.5" fill="currentColor"/>
      </svg>
    </button>
  </div>
</template>

<style scoped>
.book-card {
  aspect-ratio: 1 / 1;
  background: rgba(255, 255, 255, 0.85);
  backdrop-filter: blur(25px);
  -webkit-backdrop-filter: blur(25px);
  border-radius: 24px;
  padding: 20px 16px 18px;
  box-shadow:
    0 4px 16px rgba(0, 0, 0, 0.04),
    0 0 0 1px rgba(255, 255, 255, 0.6);
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  text-align: center;
  position: relative;
  cursor: pointer;
  transition: all 0.2s ease;
  -webkit-tap-highlight-color: transparent;
}
.book-card:active {
  transform: scale(0.97);
  background: rgba(255, 255, 255, 0.92);
}

/* ---- 3D 立体书本 ---- */
.book-3d {
  width: 72px;
  height: 72px;
  position: relative;
  margin-bottom: 14px;
  transform: perspective(400px) rotateX(5deg) rotateY(-15deg);
  transition: transform 0.25s ease;
}

/* 书脊 */
.spine {
  position: absolute;
  left: 0;
  top: 2px;
  width: 10px;
  height: 68px;
  background: #e07b1e;
  border-radius: 2px 0 0 2px;
  box-shadow: inset -2px 0 4px rgba(0, 0, 0, 0.2);
  z-index: 2;
  transform: rotateY(20deg);
  transform-origin: left center;
}

/* 书页层（侧面厚度） */
.pages {
  position: absolute;
  left: 10px;
  top: 4px;
  width: 8px;
  height: 64px;
  background: linear-gradient(180deg, #fdfaf3 0%, #e8dbc4 100%);
  border-radius: 0 1px 1px 0;
  box-shadow: inset -1px 0 2px rgba(0, 0, 0, 0.08);
  z-index: 1;
  transform: rotateY(15deg);
  transform-origin: left center;
}

/* 封面 */
.cover {
  position: absolute;
  left: 16px;
  top: 0;
  width: 56px;
  height: 72px;
  background: linear-gradient(135deg, #ffaa3a 0%, #ff8c1a 100%);
  border-radius: 2px 6px 6px 2px;
  box-shadow:
    0 4px 10px rgba(0, 0, 0, 0.15),
    inset 0 1px 0 rgba(255, 255, 255, 0.3);
  z-index: 3;
  display: flex;
  align-items: center;
  justify-content: center;
  transform: rotateY(12deg);
  transform-origin: left center;
}

.cover-icon {
  color: rgba(255, 255, 255, 0.8);
  filter: drop-shadow(0 1px 2px rgba(0, 0, 0, 0.15));
}

/* ---- 书名 ---- */
.book-name {
  font-size: 15px;
  font-weight: 590;
  color: #1d1d1f;
  letter-spacing: -0.2px;
  max-width: 100%;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  line-height: 1.35;
}

/* ---- 更多按钮 ---- */
.more-btn {
  position: absolute;
  top: 8px;
  right: 6px;
  width: 30px;
  height: 30px;
  border: none;
  background: transparent;
  color: #c7c7cc;
  display: flex;
  align-items: center;
  justify-content: center;
  cursor: pointer;
  border-radius: 50%;
  -webkit-tap-highlight-color: transparent;
  transition: background 0.15s;
}
.more-btn:active {
  background: rgba(0, 0, 0, 0.06);
  color: #8e8e93;
}
</style>
