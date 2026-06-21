<script setup lang="ts">
import {ref} from 'vue'
import {useRouter} from 'vue-router'
import {Snackbar} from '@varlet/ui'
import {useUserStore} from '@/stores/user'
import {initAdmin, queryMyInfo, updateSiteConfig, updateEmailConfig, testEmail} from '@/api'
import {markSystemInitialized} from '@/router'
import {md5} from 'js-md5'
import agreementText from '@/assets/agreement.md?raw'

const appIcon = '/app-icon.svg'
const router = useRouter()
const userStore = useUserStore()

const currentStep = ref(1)
const loading = ref(false)
const testingEmail = ref(false)

// ========== 步骤 2: 管理员账户 ==========
const adminEmail = ref('')
const adminNickname = ref('')
const adminPassword = ref('')
const adminConfirmPassword = ref('')
const showPassword = ref(false)

// ========== 步骤 3: 站点配置 ==========
const webSite = ref('')
const adminMail = ref('')
const registerEnabled = ref(false)

// ========== 步骤 4: 邮箱配置 ==========
const hostName = ref('')
const sslSmtpPort = ref<number | undefined>()
const smtpPort = ref<number | undefined>()
const ssl = ref(false)
const mailUserName = ref('')
const mailPassword = ref('')
const mailFrom = ref('')
const testTo = ref('')

// ========== Markdown 解析 ==========
function parseMarkdown(text: string): string {
  const lines = text.split('\n')
  let html = ''
  let inList = false

  for (const line of lines) {
    if (!line.trim()) {
      if (inList) { html += '</ul>'; inList = false }
      continue
    }
    if (line.startsWith('# ')) {
      if (inList) { html += '</ul>'; inList = false }
      html += `<h3>${escapeHtml(line.slice(2))}</h3>`
      continue
    }
    if (line.startsWith('## ')) {
      if (inList) { html += '</ul>'; inList = false }
      html += `<h4>${escapeHtml(line.slice(3))}</h4>`
      continue
    }
    let processed = line.replace(/\*\*(.+?)\*\*/g, '<strong>$1</strong>')
    processed = escapeHtml(processed).replace(/&lt;strong&gt;(.+?)&lt;\/strong&gt;/g, '<strong>$1</strong>')
    const numMatch = processed.match(/^(\d+)\\\.\s(.+)/)
    if (numMatch) {
      if (!inList) { html += '<ul>'; inList = true }
      html += `<li>${numMatch[2]}</li>`
      continue
    }
    if (inList) { html += '</ul>'; inList = false }
    html += `<p>${processed}</p>`
  }
  if (inList) html += '</ul>'
  return html
}

function escapeHtml(str: string): string {
  return str.replace(/&/g, '&amp;').replace(/</g, '&lt;').replace(/>/g, '&gt;')
}

const emailRegex = /^[^\s@]+@[^\s@]+\.[^\s@]+$/

// ========== 步骤操作 ==========
function onAgree() {
  currentStep.value = 2
}

async function onCreateAdmin() {
  if (!adminEmail.value.trim()) {
    Snackbar.warning('请输入邮箱')
    return
  }
  if (!emailRegex.test(adminEmail.value.trim())) {
    Snackbar.warning('邮箱格式不正确')
    return
  }
  if (!adminNickname.value.trim()) {
    Snackbar.warning('请输入昵称')
    return
  }
  if (!adminPassword.value) {
    Snackbar.warning('请输入密码')
    return
  }
  if (adminPassword.value.length < 6) {
    Snackbar.warning('密码长度不能小于6位')
    return
  }
  if (adminPassword.value !== adminConfirmPassword.value) {
    Snackbar.warning('两次输入的密码不一致')
    return
  }

  loading.value = true
  try {
    const token = await initAdmin({
      email: adminEmail.value.trim(),
      password: md5(adminPassword.value),
      nickname: adminNickname.value.trim(),
    })
    userStore.setToken(token)
    const userInfo = await queryMyInfo()
    userStore.setUserInfo(userInfo)
    markSystemInitialized()
    adminMail.value = adminEmail.value.trim()
    currentStep.value = 3
  } catch {
    // 拦截器处理
  } finally {
    loading.value = false
  }
}

async function onSaveSiteConfig() {
  if (!webSite.value.trim()) {
    Snackbar.warning('请输入站点地址')
    return
  }
  if (!adminMail.value.trim()) {
    Snackbar.warning('请输入管理员邮箱')
    return
  }
  if (!emailRegex.test(adminMail.value.trim())) {
    Snackbar.warning('管理员邮箱格式不正确')
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
    currentStep.value = 4
  } catch {
    // 拦截器处理
  } finally {
    loading.value = false
  }
}

async function onTestEmail() {
  if (!hostName.value.trim() || !sslSmtpPort.value || !smtpPort.value || !mailUserName.value.trim() || !mailPassword.value.trim() || !mailFrom.value.trim()) {
    Snackbar.warning('请先填写完整的邮箱配置')
    return
  }
  if (!emailRegex.test(mailFrom.value.trim())) {
    Snackbar.warning('发送邮箱地址格式不正确')
    return
  }
  if (!testTo.value.trim()) {
    Snackbar.warning('请输入测试收件邮箱')
    return
  }
  if (!emailRegex.test(testTo.value.trim())) {
    Snackbar.warning('测试收件邮箱格式不正确')
    return
  }

  testingEmail.value = true
  try {
    await testEmail({
      hostName: hostName.value.trim(),
      sslSmtpPort: sslSmtpPort.value,
      smtpPort: smtpPort.value,
      ssl: ssl.value,
      userName: mailUserName.value.trim(),
      password: mailPassword.value.trim(),
      from: mailFrom.value.trim(),
      testTo: testTo.value.trim(),
    })
    Snackbar.success('测试邮件发送成功，请检查收件箱')
  } catch {
    // 拦截器处理
  } finally {
    testingEmail.value = false
  }
}

async function onSaveEmailConfig() {
  if (!hostName.value.trim() || !sslSmtpPort.value || !smtpPort.value || !mailUserName.value.trim() || !mailPassword.value.trim() || !mailFrom.value.trim()) {
    Snackbar.warning('请填写完整的邮箱配置')
    return
  }
  if (!emailRegex.test(mailFrom.value.trim())) {
    Snackbar.warning('发送邮箱地址格式不正确')
    return
  }

  loading.value = true
  try {
    await updateEmailConfig({
      hostName: hostName.value.trim(),
      sslSmtpPort: sslSmtpPort.value,
      smtpPort: smtpPort.value,
      ssl: ssl.value,
      userName: mailUserName.value.trim(),
      password: mailPassword.value.trim(),
      from: mailFrom.value.trim(),
    })
    Snackbar.success('邮箱配置保存成功')
    currentStep.value = 5
  } catch {
    // 拦截器处理
  } finally {
    loading.value = false
  }
}

function onSkipEmail() {
  currentStep.value = 5
}

function onEnterSystem() {
  router.replace('/home')
}
</script>

<template>
  <div class="init-page">
    <div class="init-container">
      <!-- 品牌标识 -->
      <div class="brand">
        <div class="brand-title">
          <img :src="appIcon" alt="月衡 Logo" class="brand-logo" />
          <h1 class="brand-name">月衡</h1>
        </div>
        <p class="brand-en">Mrecord</p>
        <p class="brand-slogan">系统初始化向导</p>
      </div>

      <!-- 步骤指示器 -->
      <div v-if="currentStep <= 4" class="stepper">
        <div v-for="s in 4" :key="s" class="step-dot" :class="{ 'step-dot--active': currentStep === s, 'step-dot--done': currentStep > s }">
          <span v-if="currentStep > s" class="step-check">✓</span>
          <span v-else>{{ s }}</span>
        </div>
      </div>

      <!-- 步骤 1: 用户协议 -->
      <div v-if="currentStep === 1" class="step-content">
        <div class="agreement-card">
          <div class="agreement-title">用户协议及隐私政策</div>
          <div class="agreement-body" v-html="parseMarkdown(agreementText)"></div>
        </div>
        <button class="primary-btn" @click="onAgree">同意并继续</button>
      </div>

      <!-- 步骤 2: 管理员账户 -->
      <div v-if="currentStep === 2" class="step-content">
        <div class="step-title">创建管理员账户</div>
        <div class="step-desc">请设置系统管理员的登录信息</div>

        <div class="form-card">
          <div class="form-item">
            <label class="form-label">邮箱</label>
            <div class="input-wrapper">
              <input v-model="adminEmail" type="email" placeholder="管理员登录邮箱" class="form-input" autocomplete="email" />
            </div>
          </div>

          <div class="form-item">
            <label class="form-label">昵称</label>
            <div class="input-wrapper">
              <input v-model="adminNickname" placeholder="管理员昵称" class="form-input" maxlength="20" />
            </div>
          </div>

          <div class="form-item">
            <label class="form-label">密码</label>
            <div class="input-wrapper">
              <input v-model="adminPassword" :type="showPassword ? 'text' : 'password'" placeholder="至少6位" class="form-input" autocomplete="new-password" />
              <button class="eye-btn" @click="showPassword = !showPassword" type="button">
                <svg v-if="!showPassword" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" width="20" height="20">
                  <path d="M1 12s4-8 11-8 11 8 11 8-4 8-11 8S1 12 1 12z" />
                  <circle cx="12" cy="12" r="3" />
                </svg>
                <svg v-else viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" width="20" height="20">
                  <path d="M17.94 17.94A10.07 10.07 0 0112 20c-7 0-11-8-11-8a18.45 18.45 0 015.06-5.94" />
                  <path d="M9.9 4.24A9.12 9.12 0 0112 4c7 0 11 8 11 8a18.5 18.5 0 01-2.16 3.19" />
                  <line x1="1" y1="1" x2="23" y2="23" />
                </svg>
              </button>
            </div>
          </div>

          <div class="form-item">
            <label class="form-label">确认密码</label>
            <div class="input-wrapper">
              <input v-model="adminConfirmPassword" :type="showPassword ? 'text' : 'password'" placeholder="再次输入密码" class="form-input" autocomplete="new-password" />
            </div>
          </div>
        </div>

        <button class="primary-btn" :disabled="loading" @click="onCreateAdmin">
          {{ loading ? '创建中...' : '创建管理员' }}
        </button>
      </div>

      <!-- 步骤 3: 站点配置 -->
      <div v-if="currentStep === 3" class="step-content">
        <div class="step-title">站点信息配置</div>
        <div class="step-desc">配置站点地址和管理员联系方式</div>

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

        <button class="primary-btn" :disabled="loading" @click="onSaveSiteConfig">
          {{ loading ? '保存中...' : '保存并继续' }}
        </button>
      </div>

      <!-- 步骤 4: 邮箱配置 -->
      <div v-if="currentStep === 4" class="step-content">
        <div class="step-title">邮箱服务配置</div>
        <div class="step-desc">配置 SMTP 邮件发送服务，可稍后在管理中心配置</div>

        <div class="form-card">
          <div class="section-title">SMTP 服务器</div>

          <div class="form-item">
            <label class="form-label">SMTP 服务器地址</label>
            <div class="input-wrapper">
              <input v-model="hostName" placeholder="例如 smtp.qq.com" class="form-input" />
            </div>
          </div>

          <div class="form-row">
            <div class="form-item form-item--half">
              <label class="form-label">SSL-SMTP 端口</label>
              <div class="input-wrapper">
                <input v-model.number="sslSmtpPort" type="number" placeholder="465" class="form-input" />
              </div>
            </div>
            <div class="form-item form-item--half">
              <label class="form-label">SMTP 端口</label>
              <div class="input-wrapper">
                <input v-model.number="smtpPort" type="number" placeholder="587" class="form-input" />
              </div>
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
              <input v-model="mailUserName" placeholder="登录邮箱服务器的用户名" class="form-input" />
            </div>
          </div>

          <div class="form-item">
            <label class="form-label">邮箱授权码</label>
            <div class="input-wrapper">
              <input v-model="mailPassword" type="password" placeholder="邮箱授权码或密码" class="form-input" />
            </div>
          </div>

          <div class="form-item">
            <label class="form-label">发送邮箱地址</label>
            <div class="input-wrapper">
              <input v-model="mailFrom" type="email" placeholder="发件人邮箱地址" class="form-input" />
            </div>
          </div>
        </div>

        <div class="form-card">
          <div class="section-title">发送测试</div>
          <div class="form-item">
            <label class="form-label">测试收件邮箱</label>
            <div class="input-wrapper">
              <input v-model="testTo" type="email" placeholder="输入收件邮箱地址" class="form-input" />
            </div>
          </div>
          <button class="secondary-btn" :disabled="testingEmail" @click="onTestEmail">
            {{ testingEmail ? '发送中...' : '发送测试邮件' }}
          </button>
        </div>

        <button class="primary-btn" :disabled="loading" @click="onSaveEmailConfig">
          {{ loading ? '保存中...' : '保存并完成' }}
        </button>
        <button class="skip-btn" @click="onSkipEmail">跳过，稍后配置</button>
      </div>

      <!-- 完成页 -->
      <div v-if="currentStep === 5" class="step-content complete-content">
        <div class="complete-icon">🎉</div>
        <div class="complete-title">初始化完成</div>
        <div class="complete-desc">系统已准备就绪，现在可以开始使用了</div>
        <button class="primary-btn" @click="onEnterSystem">进入系统</button>
      </div>
    </div>
  </div>
</template>

<style scoped>
.init-page {
  min-height: 100vh;
  min-height: 100dvh;
  background: #f5f5f5;
  display: flex;
  justify-content: center;
  padding: 0 16px;
  padding-top: env(safe-area-inset-top, 0px);
  padding-bottom: calc(24px + env(safe-area-inset-bottom, 0px));
}

.init-container {
  width: 100%;
  max-width: 420px;
  padding-top: 36px;
}

/* 品牌 */
.brand {
  text-align: center;
  margin-bottom: 28px;
}
.brand-title {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  gap: 10px;
  margin-bottom: 2px;
}
.brand-logo {
  width: 42px;
  height: 42px;
  border-radius: 12px;
  box-shadow: 0 6px 16px rgba(249, 114, 22, 0.18);
  flex-shrink: 0;
}
.brand-name {
  font-size: 42px;
  font-weight: 800;
  letter-spacing: 6px;
  color: #FF6500;
  text-shadow: 0 2px 20px rgba(255, 101, 0, 0.18);
  margin-bottom: 2px;
  line-height: 1.2;
}
.brand-en {
  font-size: 15px;
  font-weight: 600;
  letter-spacing: 4px;
  color: #FF8C42;
  text-transform: uppercase;
  margin-bottom: 12px;
}
.brand-slogan {
  font-size: 13px;
  letter-spacing: 3px;
  color: #bbb;
  font-weight: 400;
}

/* 步骤指示器 */
.stepper {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 24px;
  margin-bottom: 28px;
}
.step-dot {
  width: 32px;
  height: 32px;
  border-radius: 50%;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 13px;
  font-weight: 600;
  background: #e8e8e8;
  color: #999;
  transition: all 0.3s;
  position: relative;
}
.step-dot--active {
  background: #FF6500;
  color: #fff;
  box-shadow: 0 2px 12px rgba(255, 101, 0, 0.3);
}
.step-dot--done {
  background: #FF6500;
  color: #fff;
  opacity: 0.7;
}
.step-dot + .step-dot::before {
  content: '';
  position: absolute;
  right: 100%;
  width: 24px;
  height: 2px;
  background: #e0e0e0;
  top: 50%;
  transform: translateY(-50%);
}
.step-dot--active + .step-dot::before,
.step-dot--done + .step-dot::before {
  /* no override needed — line is between dots */
}
.step-dot--done + .step-dot::before {
  background: #FF6500;
  opacity: 0.5;
}
.step-check {
  font-size: 14px;
}

/* 步骤内容 */
.step-content {
  animation: fadeIn 0.3s ease;
}
@keyframes fadeIn {
  from { opacity: 0; transform: translateY(12px); }
  to { opacity: 1; transform: translateY(0); }
}

.step-title {
  font-size: 20px;
  font-weight: 700;
  color: #333;
  text-align: center;
  margin-bottom: 6px;
}
.step-desc {
  font-size: 13px;
  color: #999;
  text-align: center;
  margin-bottom: 20px;
}

/* 协议卡片 */
.agreement-card {
  background: #fff;
  border-radius: 16px;
  box-shadow: 0 1px 4px rgba(0, 0, 0, 0.04);
  margin-bottom: 16px;
  overflow: hidden;
}
.agreement-title {
  font-size: 16px;
  font-weight: 700;
  color: #333;
  padding: 18px 20px 14px;
  border-bottom: 1px solid #f0e8e0;
  letter-spacing: 1px;
}
.agreement-body {
  max-height: 45vh;
  overflow-y: auto;
  padding: 16px 20px 20px;
  font-size: 13px;
  line-height: 1.8;
  color: #444;
}
.agreement-body :deep(h3) {
  font-size: 15px;
  font-weight: 700;
  color: #FF6500;
  margin: 18px 0 8px;
  padding-bottom: 6px;
  border-bottom: 1px solid #f5ede6;
}
.agreement-body :deep(h3:first-child) { margin-top: 0; }
.agreement-body :deep(h4) {
  font-size: 14px;
  font-weight: 600;
  color: #FF8C42;
  margin: 12px 0 6px;
}
.agreement-body :deep(p) { margin: 0 0 6px; text-align: justify; }
.agreement-body :deep(ul) { margin: 4px 0 6px; padding-left: 16px; list-style: none; }
.agreement-body :deep(li) { position: relative; padding-left: 6px; margin-bottom: 3px; }
.agreement-body :deep(strong) { color: #333; font-weight: 600; }

/* 表单卡片 */
.form-card {
  background: #fff;
  border-radius: 16px;
  padding: 20px 16px;
  box-shadow: 0 1px 4px rgba(0, 0, 0, 0.04);
  margin-bottom: 12px;
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
.form-item:last-child { margin-bottom: 0; }
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
  min-width: 0;
}
.form-input::placeholder { color: #bbb; }

.form-row {
  display: flex;
  gap: 12px;
}
.form-item--half {
  flex: 1;
  min-width: 0;
}

/* 开关行 */
.switch-row {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 12px;
}
.switch-info { flex: 1; min-width: 0; }
.switch-label { font-size: 15px; color: #333; font-weight: 500; }
.switch-desc { font-size: 12px; color: #999; margin-top: 2px; }

/* 密码显隐 */
.eye-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 36px;
  height: 36px;
  border-radius: 10px;
  background: none;
  border: none;
  cursor: pointer;
  flex-shrink: 0;
  padding: 0;
  color: #c0a080;
  transition: color 0.2s;
}
.eye-btn:active { color: #FF6500; }

/* 按钮 */
.primary-btn {
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
  margin-top: 4px;
}
.primary-btn:active:not(:disabled) {
  background: #e05800;
  transform: scale(0.98);
}
.primary-btn:disabled { opacity: 0.7; cursor: not-allowed; }

.secondary-btn {
  width: 100%;
  height: 44px;
  border: 1.5px solid #FF6500;
  border-radius: 12px;
  background: transparent;
  color: #FF6500;
  font-size: 14px;
  font-weight: 600;
  cursor: pointer;
  transition: all 0.2s;
  margin-top: 4px;
}
.secondary-btn:active:not(:disabled) {
  background: rgba(255, 101, 0, 0.06);
  transform: scale(0.98);
}
.secondary-btn:disabled { opacity: 0.6; cursor: not-allowed; }

.skip-btn {
  width: 100%;
  height: 44px;
  border: none;
  background: transparent;
  color: #999;
  font-size: 14px;
  cursor: pointer;
  margin-top: 4px;
  transition: color 0.2s;
}
.skip-btn:active { color: #FF6500; }

/* 完成页 */
.complete-content {
  text-align: center;
  padding-top: 48px;
}
.complete-icon {
  font-size: 64px;
  margin-bottom: 20px;
}
.complete-title {
  font-size: 24px;
  font-weight: 700;
  color: #333;
  margin-bottom: 10px;
}
.complete-desc {
  font-size: 14px;
  color: #999;
  margin-bottom: 40px;
}
</style>