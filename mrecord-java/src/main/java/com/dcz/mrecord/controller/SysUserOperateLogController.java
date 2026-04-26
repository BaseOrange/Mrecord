package com.dcz.mrecord.controller;

import com.dcz.mrecord.common.CheckAdmin;
import com.dcz.mrecord.common.Result;
import com.dcz.mrecord.dto.PageInfoDTO;
import com.dcz.mrecord.entity.SysUserOperateLog;
import com.dcz.mrecord.service.SysUserOperateLogService;
import com.mybatisflex.core.paginate.Page;
import jakarta.annotation.Resource;
import org.springframework.web.bind.annotation.PostMapping;
import org.springframework.web.bind.annotation.RequestBody;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.RestController;

/**
 * 用户操作审计日志控制器
 *
 * @author dcz
 * @since 2026/04/09
 */
@RestController
@RequestMapping("/operateLog")
public class SysUserOperateLogController {
    @Resource
    private SysUserOperateLogService sysUserOperateLogService;

    /**
     * 分页查询用户操作审计日志列表
     *
     * @param queryDTO 查询参数
     * @return 用户操作审计日志列表
     */
    @CheckAdmin
    @PostMapping("/list")
    public Result<Page<SysUserOperateLog>> list(@RequestBody PageInfoDTO queryDTO) {
        Page<SysUserOperateLog> sysUserOperateLogPage = sysUserOperateLogService.queryList(queryDTO);
        return Result.success(sysUserOperateLogPage);
    }
}
