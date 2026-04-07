package com.dcz.mrecord.entity.backup;

import com.dcz.mrecord.entity.FinBook;
import com.mybatisflex.annotation.Table;
import lombok.Data;
import lombok.EqualsAndHashCode;

/**
 * 财务账簿备份实体类
 * 存储用户创建的年度/分类账簿，实现多账簿独立管理
 *
 * @author dcz
 * @since 2026/04/07
 */
@Data
@EqualsAndHashCode(callSuper = true)
@Table(value = "SYS_BACKUP_BOOK")
public class SysBackupBook extends FinBook {
}
