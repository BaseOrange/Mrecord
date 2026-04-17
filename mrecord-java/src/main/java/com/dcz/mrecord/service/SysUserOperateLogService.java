package com.dcz.mrecord.service;

import com.dcz.mrecord.dto.PageInfoDTO;
import com.dcz.mrecord.entity.SysUserOperateLog;
import com.mybatisflex.core.paginate.Page;
import com.mybatisflex.core.service.IService;

/**
 * 用户操作审计日志服务
 *
 * @author dcz
 * @since 2026/04/09
 */
public interface SysUserOperateLogService extends IService<SysUserOperateLog> {

    /**
     * 保存日志
     *
     * @param log 日志
     */
    void saveLog(SysUserOperateLog log);

    /**
     * 查询日志列表
     *
     * @param pageInfoDTO 分页参数
     * @return 日志列表
     */
    Page<SysUserOperateLog> queryList(PageInfoDTO pageInfoDTO);
}
