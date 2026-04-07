package com.dcz.mrecord.entity.backup;

import com.dcz.mrecord.entity.FinMonthItemRecord;
import com.mybatisflex.annotation.Table;
import lombok.Data;
import lombok.EqualsAndHashCode;

/**
 * 月度财务明细项备份实体类
 * 存储每月各记账项的具体金额，与月度汇总表一一对应
 *
 * @author dcz
 * @since 2026/04/07
 */
@Data
@EqualsAndHashCode(callSuper = true)
@Table(value = "SYS_BACKUP_MONTH_ITEM_RECORD")
public class SysBackupMonthItemRecord extends FinMonthItemRecord {
}
