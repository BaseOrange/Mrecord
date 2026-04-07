package com.dcz.mrecord.entity;

import com.mybatisflex.annotation.Column;
import com.mybatisflex.annotation.Table;
import lombok.Data;
import lombok.EqualsAndHashCode;

/**
 * 异步导出任务实体类
 * 存储用户Excel导出异步任务，实现非阻塞导出，任务完成后邮件通知用户
 *
 * @author dcz
 * @since 2026/04/07
 */
@Data
@EqualsAndHashCode(callSuper = true)
@Table(value = "SYS_EXPORT_TASK")
public class SysExportTask extends BaseEntity {

    /**
     * 操作用户ID，关联SYS_USER.MR_ID
     */
    @Column(value = "MR_USER_ID")
    private String userId;

    /**
     * 导出账簿ID，关联FIN_BOOK.MR_ID
     */
    @Column(value = "MR_BOOK_ID")
    private String bookId;

    /**
     * 账簿类型（YEARLY-年度，CATEGORY-分类）
     */
    @Column(value = "MR_BOOK_TYPE")
    private String bookType;

    /**
     * 导出开始年月，格式yyyyMM
     */
    @Column(value = "MR_START_YEAR_MONTH")
    private String startYearMonth;

    /**
     * 导出结束年月，格式yyyyMM
     */
    @Column(value = "MR_END_YEAR_MONTH")
    private String endYearMonth;

    /**
     * 任务状态（WAIT-待执行，RUN-执行中，SUCCESS-成功，FAIL-失败）
     */
    @Column(value = "MR_STATUS")
    private String status;

    /**
     * 生成的Excel文件名
     */
    @Column(value = "MR_FILE_NAME")
    private String fileName;

    /**
     * 任务失败原因，失败时填充
     */
    @Column(value = "MR_FAIL_REASON")
    private String failReason;
}
