package com.dcz.mrecord.dto;

import lombok.Data;

/**
 * 诊断信息响应 DTO
 *
 * <p>供前端「我的 → 点击顶部标题 5 次」打开的诊断面板展示，用于用户在 GitHub 提 issue
 * 时一键复制环境信息，帮助开发者快速定位问题。</p>
 *
 * <p>【隐私红线】月衡是记账应用，诊断信息会被用户粘贴到<b>公开</b>的 GitHub issue
 * 里，因此本 DTO 只包含环境元数据与<b>计数</b>：</p>
 * <ul>
 *   <li>绝不包含任何金额、资产/负债数值；</li>
 *   <li>绝不包含邮箱、昵称、手机号、密码、token；</li>
 *   <li>绝不包含账簿名、模板条目名、备注内容。</li>
 * </ul>
 *
 * <p>字段与 Rust 版（mrecord-rust/src/model/diagnostic.rs）一一对应，
 * 前端共用同一套类型；{@link #backend} 标明后端实现类型，便于区分 Docker 版（Java）
 * 与 fnOS 版（Rust）。</p>
 *
 * @author dcz
 * @since 2026/09/21
 */
@Data
public class DiagnosticDTO {

    /**
     * 后端实现类型：Java 版固定 "java"（Rust 版为 "rust"）
     */
    private String backend;

    /**
     * 后端框架：如 "Spring Boot 3.x"
     */
    private String backendFramework;

    /**
     * 后端应用版本（与 pom.xml / manifest 一致）
     */
    private String appVersion;

    /**
     * 后端构建时间（UTC）
     */
    private String buildTime;

    /**
     * 后端进程已运行时长（秒）
     */
    private Long uptimeSecs;

    /**
     * 宿主操作系统
     */
    private String os;

    /**
     * CPU 架构
     */
    private String arch;

    /**
     * 部署模式：Java 版固定 standalone（Docker / 独立部署）
     */
    private String deployMode;

    /**
     * 数据库类型
     */
    private String dbType;

    /**
     * 数据库文件大小（字节）
     */
    private Long dbSizeBytes;

    /**
     * 当前登录用户的数据规模计数（不含金额）
     */
    private DataCounts dataCounts;

    /**
     * 当前登录用户的数据规模计数（只有计数，不含任何金额或名称）
     */
    @Data
    public static class DataCounts {

        /**
         * 账簿数量
         */
        private Long bookCount;

        /**
         * 模板条目数量
         */
        private Long templateItemCount;

        /**
         * 月度汇总记录数量
         */
        private Long monthRecordCount;

        /**
         * 月度明细记录数量
         */
        private Long monthItemCount;
    }
}
