/**
 * 简易 Markdown → HTML 转换。
 *
 * 仅用于渲染本地静态 `src/assets/agreement.md`（用户协议 / 隐私政策），
 * 不引入完整 markdown 实现。支持：h3/h4、加粗、有序列表、段落。
 *
 * 安全性：所有文本先经 `escapeHtml` 转义后再放行，`**bold**` 的 `<strong>`
 * 标签是在转义**之后**重新还原的白名单标签，因此不存在注入（见 TODO「经核实
 * 不存在的问题」一节）。
 */

function escapeHtml(str: string): string {
    return str
        .replace(/&/g, '&amp;')
        .replace(/</g, '&lt;')
        .replace(/>/g, '&gt;')
}

/**
 * 将简易 Markdown 文本转换为 HTML。
 *
 * @param text Markdown 原文
 * @returns 可直接用于 `v-html` 的 HTML（已转义）
 */
export function parseMarkdown(text: string): string {
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
