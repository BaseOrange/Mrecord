<script setup lang="ts">
import {onMounted, ref} from 'vue'
import {useRouter} from 'vue-router'
import {Snackbar} from '@varlet/ui'
import {getEmailConfig, updateEmailConfig} from '@/api'

const router = useRouter()
const loading = ref(false)
const fetching = ref(true)

const hostName = ref('')
const sslSmtpPort = ref<number | undefined>()
const smtpPort = ref<number | undefined>()
const ssl = ref(false)
const userName = ref('')
const password = ref('')
const from = ref('')

onMounted(async () => {
  try {
    const config = await getEmailConfig()
    if (config) {
      hostName.value = config.hostName || ''
      sslSmtpPort.value = config.sslSmtpPort
      smtpPort.value = config.smtpPort
      ssl.value = config.ssl ?? false
      userName.value = config.username || ''
      password.value = config.password || ''
      from.value = config.from || ''
    }
  } catch {
    // 拦截器处理
  } finally {
    fetching.value = false
  }
})

const onSubmit = async () => {
  if (!hostName.value.trim()) {
    Snackbar.warning('请输入SMTP服务器地址')
    return
  }
  if (!sslSmtpPort.value) {
    Snackbar.warning('请输入SSL-SMTP端口')
    return
  }
  if (!smtpPort.value) {
    Snackbar.warning('请输入SMTP端口')
    return
  }
  if (!userName.value.trim()) {
    Snackbar.warning('请输入邮箱用户名')
    return
  }
  if (!password.value.trim()) {
    Snackbar.warning('请输入邮箱授权码')
    return
  }
  if (!from.value.trim()) {
    Snackbar.warning('请输入发送邮箱地址')
    return
  }

  loading.value = true
  try {
    await updateEmailConfig({
      hostName: hostName.value.trim(),
      sslSmtpPort: sslSmtpPort.value,
      smtpPort: smtpPort.value,
      ssl: ssl.value,
      userName: userName.value.trim(),
      password: password.value.trim(),
      from: from.value.trim(),
    })
    Snackbar.success('邮箱配置保存成功')
    router.back()
  } catch {
    // 拦截器处理
  } finally {
    loading.value = false
  }
}
</script>

<template>
  <div class="admin-email-page">
    <div class="page-header">
      <button class="back-btn" @click="router.back()">
        <svg viewBox="0 0 24 24" width="18" height="18" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round">
          <path d="M15 18l-6-6 6-6" />
        </svg>
      </button>
      <h2>邮箱服务配置</h2>
      <span class="header-spacer"></span>
    </div>

    <div class="page-body">
      <template v-if="!fetching">
        <div class="form-card">
          <div class="section-title">SMTP 服务器</div>

          <div class="form-item">
            <label class="form-label">SMTP 服务器地址</label>
            <div class="input-wrapper">
              <input v-model="hostName" placeholder="例如 smtp.qq.com" class="form-input" />
            </div>
          </div>

          <div class="form-item">
            <label class="form-label">SSL-SMTP 端口</label>
            <div class="input-wrapper">
              <input v-model.number="sslSmtpPort" type="number" placeholder="例如 465" class="form-input" />
            </div>
          </div>

          <div class="form-item">
            <label class="form-label">SMTP 端口</label>
            <div class="input-wrapper">
              <input v-model.number="smtpPort" type="number" placeholder="例如 587" class="form-input" />
            </div>
          </div>

          <div class="form-item">
            <div class="switch-row">
              <div class="switch-info">
                <div class="switch-label">启用 SSL</div>
                <div class="switch-desc">开启 SSL 加密连接</div>
              </div>
              <var-switch v-model="ssl" :color="'#FF6500'" :close-color="'#e0e0e0'" size="22" />
            </div>
          </div>
        </div>

        <div class="form-card">
          <div class="section-title">账号信息</div>

          <div class="form-item">
            <label class="form-label">邮箱用户名</label>
            <div class="input-wrapper">
              <input v-model="userName" placeholder="登录邮箱服务器的用户名" class="form-input" />
            </div>
          </div>

          <div class="form-item">
            <label class="form-label">邮箱授权码</label>
            <div class="input-wrapper">
              <input v-model="password" type="password" placeholder="邮箱授权码或密码" class="form-input" />
            </div>
          </div>

          <div class="form-item">
            <label class="form-label">发送邮箱地址</label>
            <div class="input-wrapper">
              <input v-model="from" type="email" placeholder="发件人邮箱地址" class="form-input" />
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
.admin-email-page {
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

.section-title {
  font-size: 15px;
  font-weight: 600;
  color: #333;
  margin-bottom: 16px;
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
