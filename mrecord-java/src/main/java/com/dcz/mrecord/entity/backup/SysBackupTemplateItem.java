package com.dcz.mrecord.entity.backup;

import com.dcz.mrecord.entity.FinTemplateItem;
import com.mybatisflex.annotation.Table;
import lombok.Data;
import lombok.EqualsAndHashCode;

/**
 * 记账模板明细备份实体类
 *
 * @author dcz
 * @since 2026/04/07
 */
@Data
@EqualsAndHashCode(callSuper = true)
@Table(value = "SYS_BACKUP_TEMPLATE_ITEM")
public class SysBackupTemplateItem extends FinTemplateItem {
}
