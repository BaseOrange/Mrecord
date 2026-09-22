package com.dcz.mrecord.controller;

import com.dcz.mrecord.common.Result;
import com.dcz.mrecord.common.UserContext;
import com.dcz.mrecord.dto.DiagnosticDTO;
import jakarta.annotation.Resource;
import lombok.extern.slf4j.Slf4j;
import org.springframework.beans.factory.annotation.Value;
import org.springframework.boot.SpringBootVersion;
import org.springframework.jdbc.core.JdbcTemplate;
import org.springframework.web.bind.annotation.PostMapping;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.RestController;

import java.io.File;
import java.lang.management.ManagementFactory;
import java.net.URL;
import java.time.Instant;
import java.time.ZoneId;
import java.time.format.DateTimeFormatter;
import java.util.Collections;
import java.util.List;
/**
 * 诊断信息控制器
 *
 * <p>对应 Rust 版 {@code mrecord-rust/src/handler/diagnostic.rs}。前端「我的」页面顶部
 * 标题点击 5 次后打开诊断面板，调用本接口获取后端环境信息，供用户在 GitHub 提 issue
 * 时一键复制。</p>
 *
 * <p>【设计要点】</p>
 * <ul>
 *   <li>需要登录：{@link UserContext#getUserId()} 由登录拦截器注入，数据计数按当前
 *       用户过滤；未登录场景（如登录页白屏）下前端纯环境信息已足够排障。</li>
 *   <li>只返回环境元数据与<b>计数</b>：绝不包含金额、账号、账簿名等业务或敏感数据
 *       （详见 {@link DiagnosticDTO} 的隐私红线说明）。</li>
 *   <li>本接口被操作日志拦截器排除（WebConfig 中配置），查看诊断信息本身不会
 *       产生审计日志，避免「查看 → 产生日志」的循环。</li>
 * </ul>
 *
 * @author dcz
 * @since 2026/09/21
 */
@Slf4j
@RestController
@RequestMapping("/diagnostic")
public class DiagnosticController {

    @Resource
    private JdbcTemplate jdbcTemplate;

    /**
     * 应用版本：取自 application.yml 的 mrecord.app-version，与 pom.xml / manifest 对齐
     */
    @Value("${mrecord.app-version:2.1.0}")
    private String appVersion;

    /**
     * 数据源地址（形如 jdbc:sqlite:./data/mrecord.db），用于定位数据库文件计算大小
     */
    @Value("${spring.datasource.url}")
    private String datasourceUrl;

    /**
     * 查询诊断信息
     *
     * @return 诊断信息
     */
    @PostMapping("/query")
    public Result<DiagnosticDTO> query() {
        String userId = UserContext.getUserId();

        DiagnosticDTO dto = new DiagnosticDTO();
        dto.setBackend("java");
        dto.setBackendFramework("Spring Boot " + SpringBootVersion.getVersion());
        dto.setAppVersion(appVersion);
        dto.setBuildTime(readBuildTime());
        // getUptime 返回毫秒，换算为秒
        dto.setUptimeSecs(ManagementFactory.getRuntimeMXBean().getUptime() / 1000);
        dto.setOs(System.getProperty("os.name"));
        dto.setArch(System.getProperty("os.arch"));
        // Java 版只以 Docker / 独立部署形态运行，不支持飞牛网关模式
        dto.setDeployMode("standalone");
        dto.setDbType("sqlite");
        dto.setDbSizeBytes(readDbSizeBytes());
        dto.setDataCounts(queryDataCounts(userId));

        return Result.success(dto);
    }

    /**
     * 查询当前用户的数据规模计数
     *
     * <p>FIN_TEMPLATE_ITEM 与 FIN_MONTH_ITEM_RECORD 只存 MR_BOOK_ID（通过账簿间接关联
     * 用户），需要先拿到账簿 id 集合再 IN 查询——与 Rust 版逻辑同构。</p>
     *
     * @param userId 当前登录用户 ID
     * @return 数据规模计数
     */
    private DiagnosticDTO.DataCounts queryDataCounts(String userId) {
        DiagnosticDTO.DataCounts counts = new DiagnosticDTO.DataCounts();

        Long bookCount = jdbcTemplate.queryForObject(
                "SELECT COUNT(*) FROM FIN_BOOK WHERE MR_USER_ID = ? AND MR_IS_DELETED = 0",
                Long.class, userId);
        counts.setBookCount(bookCount != null ? bookCount : 0L);

        Long monthRecordCount = jdbcTemplate.queryForObject(
                "SELECT COUNT(*) FROM FIN_MONTH_RECORD WHERE MR_USER_ID = ? AND MR_IS_DELETED = 0",
                Long.class, userId);
        counts.setMonthRecordCount(monthRecordCount != null ? monthRecordCount : 0L);

        // 先取账簿 id 集合
        List<String> bookIds = jdbcTemplate.queryForList(
                "SELECT MR_ID FROM FIN_BOOK WHERE MR_USER_ID = ? AND MR_IS_DELETED = 0",
                String.class, userId);

        if (bookIds.isEmpty()) {
            // 没有账簿时关联计数必为 0，直接短路，避免空 IN 查询
            counts.setTemplateItemCount(0L);
            counts.setMonthItemCount(0L);
            return counts;
        }

        // 动态占位符：IN 子句参数个数与 bookIds 数量一致
        String placeholders = String.join(",", Collections.nCopies(bookIds.size(), "?"));
        Long templateItemCount = jdbcTemplate.queryForObject(
                "SELECT COUNT(*) FROM FIN_TEMPLATE_ITEM WHERE MR_IS_DELETED = 0 AND MR_BOOK_ID IN ("
                        + placeholders + ")",
                Long.class, bookIds.toArray());
        counts.setTemplateItemCount(templateItemCount != null ? templateItemCount : 0L);

        Long monthItemCount = jdbcTemplate.queryForObject(
                "SELECT COUNT(*) FROM FIN_MONTH_ITEM_RECORD WHERE MR_IS_DELETED = 0 AND MR_BOOK_ID IN ("
                        + placeholders + ")",
                Long.class, bookIds.toArray());
        counts.setMonthItemCount(monthItemCount != null ? monthItemCount : 0L);

        return counts;
    }

    /**
     * 读取数据库文件大小
     *
     * <p>从数据源 URL（jdbc:sqlite:./data/mrecord.db）解析文件路径；读取失败时返回 0
     * 而非抛异常——诊断信息应尽可能可用，文件大小不是关键字段。</p>
     *
     * @return 数据库文件字节数
     */
    private long readDbSizeBytes() {
        try {
            // JDBC URL 前缀 "jdbc:sqlite:" 之后即文件路径
            String path = datasourceUrl.substring("jdbc:sqlite:".length())
                    // SQLite URL 可能带 ? 附加参数（如 ?busy_timeout=5000）
                    .split("\\?")[0];
            File dbFile = new File(path);
            if (dbFile.exists()) {
                return dbFile.length();
            }
        } catch (Exception e) {
            log.warn("读取数据库文件大小失败: {}", e.getMessage());
        }
        return 0L;
    }

    /**
     * 读取后端构建时间
     *
     * <p>优先读 MANIFEST.MF 的 {@code Build-Time}（由 maven-jar-plugin 在构建期写入，
     * spring-boot repackage 会把它合并进最终 fat jar，生产 jar 一定有值）；
     * 读不到时退化为类所在 jar / classes 目录的最后修改时间（开发模式近似值）；
     * 都失败时返回 "unknown"，不阻断诊断接口。</p>
     *
     * <p>【时区】Maven 时间戳是构建机本地时间，这里按「构建机与运行机同一时区」
     * 的常见假设转成 UTC 输出，与 Rust 端格式统一；构建时间只用于区分构建版本，
     * 跨时区的秒级偏差不影响排障。</p>
     *
     * @return 构建时间（UTC 字符串）
     */
    private String readBuildTime() {
        // 1. 优先读 MANIFEST.MF
        try (var is = DiagnosticController.class.getResourceAsStream("/META-INF/MANIFEST.MF")) {
            if (is != null) {
                var attrs = new java.util.jar.Manifest(is).getMainAttributes();
                String buildTime = attrs.getValue("Build-Time");
                if (buildTime != null && !buildTime.isEmpty()) {
                    // Maven 的 ${maven.build.timestamp} 使用 UTC，直接拼接 UTC 后缀，
                    // 与 Rust 端 "yyyy-MM-dd HH:mm:ss 'UTC'" 格式对齐
                    return buildTime + " UTC";
                }
            }
        } catch (Exception e) {
            log.warn("读取 MANIFEST.MF 构建时间失败: {}", e.getMessage());
        }

        // 2. 退化方案：jar / classes 目录的最后修改时间
        try {
            URL location = DiagnosticController.class
                    .getProtectionDomain()
                    .getCodeSource()
                    .getLocation();
            if (location != null) {
                String spec = location.toString();
                // Spring Boot fat jar 内的类，location 形如 jar:file:/...jar!/BOOT-INF/classes!/
                if (spec.startsWith("jar:")) {
                    spec = spec.substring("jar:".length());
                    int bang = spec.indexOf('!');
                    if (bang > 0) {
                        spec = spec.substring(0, bang);
                    }
                }
                if (spec.startsWith("file:")) {
                    spec = spec.substring("file:".length());
                }
                File file = new File(spec);
                if (file.exists()) {
                    return Instant.ofEpochMilli(file.lastModified())
                            .atZone(ZoneId.of("UTC"))
                            .format(DateTimeFormatter.ofPattern("yyyy-MM-dd HH:mm:ss 'UTC'"));
                }
            }
        } catch (Exception e) {
            log.warn("读取类路径构建时间失败: {}", e.getMessage());
        }
        return "unknown";
    }
}
