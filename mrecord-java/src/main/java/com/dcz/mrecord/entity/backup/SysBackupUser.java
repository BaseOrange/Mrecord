package com.dcz.mrecord.entity.backup;

import com.dcz.mrecord.entity.SysUser;
import com.mybatisflex.annotation.Table;
import lombok.Data;
import lombok.EqualsAndHashCode;

/**
 * 用户备份实体类
 *
 * @author dcz
 * @since 2026/04/07
 */
@Data
@EqualsAndHashCode(callSuper = true)
@Table(value = "SYS_BACKUP_USER")
public class SysBackupUser extends SysUser {
}
