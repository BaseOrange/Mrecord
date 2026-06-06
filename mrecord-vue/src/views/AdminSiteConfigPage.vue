<script setup lang="ts">
import {onMounted, ref} from 'vue'
import {useRouter} from 'vue-router'
import {Snackbar} from '@varlet/ui'
import {getSiteConfig, updateSiteConfig} from '@/api'

const router = useRouter()
const loading = ref(false)
const fetching = ref(true)

const webSite = ref('')
const adminMail = ref('')
const registerEnabled = ref(false)

onMounted(async () => {
  try {
    const config = await getSiteConfig()
    webSite.value = config.webSite || ''
    adminMail.value = config.adminMail || ''
    registerEnabled.value = config.registerEnabled ?? false
  } catch {
    // 拦截器处理
  } finally {
    fetching.value = false
  }
})

const onSubmit = async () => {
  if (!webSite.value.trim()) {
    Snackbar.warning('请输入站点地址')
    return
  }
  if (!adminMail.value.trim()) {
    Snackbar.warning('请输入管理员邮箱')
    return
  }

  loading.value = true
  try {
    await updateSiteConfig({
      webSite: webSite.value.trim(),
      adminMail: adminMail.value.trim(),
      registerEnabled: registerEnabled.value,
    })
    Snackbar.success('站点配置保存成功')
    router.back()
  } catch {
    // 拦截器处理
  } finally {
    loading.value = false
  }
}
</script>

<template>
  <div class="admin-site-page">
    <div class="page-header">
      <button class="back-btn" @click="router.back()">
        <svg viewBox="0 0 24 24" width="18" height="18" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round">
          <path d="M15 18l-6-6 6-6" />
        </svg>
      </button>
      <h2>站点信息配置</h2>
      <span class="header-spacer"></span>
    </div>

    <div class="page-body">
      <template v-if="!fetching">
        <div class="form-card">
          <div class="form-item">
            <label class="form-label">站点地址</label>
            <div class="input-wrapper">
              <input v-model="webSite" placeholder="例如 https://example.com" class="form-input" />
            </div>
          </div>

          <div class="form-item">
            <label class="form-label">管理员邮箱</label>
            <div class="input-wrapper">
              <input v-model="adminMail" type="email" placeholder="接收系统通知的管理员邮箱" class="form-input" />
            </div>
          </div>

          <div class="form-item">
            <div class="switch-row">
              <div class="switch-info">
                <div class="switch-label">开放注册</div>
                <div class="switch-desc">关闭后登录页将不显示注册入口</div>
              </div>
              <var-switch v-model="registerEnabled" :color="'#FF6500'" :close-color="'#e0e0e0'" size="22" />
            </div>
          </div>
        </div>

        <button
          class="submit-btn"
          :class="{ 'submit-btn--loading': loading }"
          :disabled="loading"
          @click="onSubmit"
        >
          {{ loading ? '保存中...' : '保存' }}
        </button>
      </template>

      <div v-else class="loading-wrapper">
        <var-loading type="circle" color="#FF6500" />
      </div>
    </div>
  </div>
</template>

<style scoped>
.admin-site-page {
  min-height: 100vh;
  background: #f5f5f5;
  padding-bottom: calc(24px + env(safe-area-inset-bottom, 0px));
}

.page-header {
  background: #fff;
  padding: calc(16px + env(safe-area-inset-top, 0px)) 16px 16px;
  display: flex;
  align-items: center;
  justify-content: space-between;
  border-bottom: 1px solid #f0f0f0;
}
.page-header h2 {
  font-size: 18px;
  font-weight: 600;
  color: #333;
  margin: 0;
  line-height: 1;
}
.back-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 36px;
  border-radius: 50%;
  border: none;
  background: transparent;
  color: #333;
  cursor: pointer;
  transition: background 0.15s;
  padding: 0;
}
.back-btn:active {
  background: rgba(0, 0, 0, 0.06);
  color: #FF6500;
}
.header-spacer {
  width: 36px;
}

.page-body {
  padding: 16px;
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.form-card {
  background: #fff;
  border-radius: 16px;
  padding: 20px 16px;
  box-shadow: 0 1px 4px rgba(0, 0, 0, 0.04);
}

.form-item {
  margin-bottom: 16px;
}
.form-item:last-child {
  margin-bottom: 0;
}
.form-label {
  display: block;
  font-size: 13px;
  color: #666;
  font-weight: 500;
  margin-bottom: 8px;
}

.input-wrapper {
  position: relative;
  display: flex;
  align-items: center;
  background: #fafafa;
  border: 1px solid #e8e8e8;
  border-radius: 12px;
  padding: 0 12px;
  height: 48px;
  transition: all 0.2s;
}
.input-wrapper:focus-within {
  border-color: #FF6500;
  background: #fff;
}

.form-input {
  flex: 1;
  height: 100%;
  font-size: 15px;
  color: #333;
  background: transparent;
  border: none;
  outline: none;
  letter-spacing: 0.5px;
}
.form-input::placeholder {
  color: #bbb;
}

.switch-row {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 12px;
}
.switch-info {
  flex: 1;
  min-width: 0;
}
.switch-label {
  font-size: 15px;
  color: #333;
  font-weight: 500;
}
.switch-desc {
  font-size: 12px;
  color: #999;
  margin-top: 2px;
}

.submit-btn {
  width: 100%;
  height: 48px;
  border: none;
  border-radius: 14px;
  background: #FF6500;
  color: #fff;
  font-size: 16px;
  font-weight: 600;
  cursor: pointer;
  transition: all 0.2s;
  -webkit-tap-highlight-color: transparent;
}
.submit-btn:active:not(:disabled) {
  background: #e05800;
  transform: scale(0.98);
}
.submit-btn--loading {
  opacity: 0.7;
}

.loading-wrapper {
  display: flex;
  justify-content: center;
  padding: 60px 0;
}
</style>
