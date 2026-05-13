# 安全配置建议

## 内容安全策略 (CSP)

建议在服务器端添加以下 CSP 头以增强安全性：

```
Content-Security-Policy: 
  default-src 'self';
  script-src 'self' 'unsafe-inline' 'unsafe-eval';
  style-src 'self' 'unsafe-inline' https://fonts.googleapis.com;
  img-src 'self' data: https:;
  font-src 'self' https://fonts.gstatic.com;
  connect-src 'self' https://api.mrecord.com;
  frame-ancestors 'none';
  base-uri 'self';
  form-action 'self';
```

## HTTP 安全头

建议在服务器端添加以下安全头：

```
X-Frame-Options: DENY
X-Content-Type-Options: nosniff
X-XSS-Protection: 1; mode=block
Strict-Transport-Security: max-age=31536000; includeSubDomains
Referrer-Policy: strict-origin-when-cross-origin
Permissions-Policy: camera=(), microphone=(), geolocation=()
```

## 前端安全最佳实践

1. **密码安全**
   - 使用 MD5 加密传输（当前实现）
   - 建议后端使用 bcrypt 或 argon2 进行哈希存储
   - 实施密码强度验证

2. **Token 管理**
   - Token 已加密存储在 localStorage
   - 设置合理的 Token 过期时间
   - 实施 Refresh Token 机制

3. **XSS 防护**
   - 用户输入已进行 sanitization
   - 避免使用 v-html 渲染不可信内容
   - Vue 的模板引擎默认转义输出

4. **CSRF 防护**
   - 使用 Authorization header 而非 Cookie
   - 实施 SameSite Cookie 策略

5. **数据验证**
   - 前后端都应进行数据验证
   - 使用 TypeScript 提供类型安全
   - 实施输入长度和格式限制

## 依赖安全

定期运行以下命令检查依赖漏洞：

```bash
yarn audit
npm audit
```

建议启用自动安全更新：
- GitHub: Dependabot
- GitLab: Dependency Scanning
