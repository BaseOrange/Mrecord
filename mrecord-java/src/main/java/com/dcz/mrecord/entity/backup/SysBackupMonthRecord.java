package com.dcz.mrecord.entity.backup;

import com.dcz.mrecord.entity.FinMonthRecord;
import com.mybatisflex.annotation.Table;
import lombok.Data;
import lombok.EqualsAndHashCode;

/**
 * 月度财务汇总备份实体类
 * 存储每月财务数据汇总指标，自动计算总资产、总负债、净资产及同比环比
 *
 * @author dcz
 * @since 2026/04/07
 */
@Data
@EqualsAndHashCode(callSuper = true)
@Table(value = "SYS_BACKUP_MONTH_RECORD")
public class SysBackupMonthRecord extends FinMonthRecord {
}
