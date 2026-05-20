package com.dcz.mrecord.service.impl;

import cn.hutool.core.util.IdUtil;
import com.dcz.mrecord.dto.PageInfoDTO;
import com.dcz.mrecord.entity.SysUserOperateLog;
import com.dcz.mrecord.mapper.SysUserOperateLogMapper;
import com.dcz.mrecord.service.SysUserOperateLogService;
import com.mybatisflex.core.paginate.Page;
import com.mybatisflex.core.query.QueryWrapper;
import com.mybatisflex.spring.service.impl.ServiceImpl;
import jakarta.annotation.Resource;
import org.springframework.stereotype.Service;

/**
 * 用户操作审计日志服务实现
 *
 * @author dcz
 * @since 2026/04/09
 */
@Service
public class SysUserOperateLogServiceImpl extends ServiceImpl<SysUserOperateLogMapper, SysUserOperateLog> implements SysUserOperateLogService {

    @Resource
    private SysUserOperateLogMapper sysUserOperateLogMapper;

    /**
     * 保存日志
     *
     * @param log 日志
     */
    @Override
    public void saveLog(SysUserOperateLog log) {
        log.setId(IdUtil.simpleUUID());
        sysUserOperateLogMapper.insert(log);
    }

    /**
     * 查询日志列表
     *
     * @param pageInfoDTO 分页参数
     * @return 日志列表
     */
    @Override
    public Page<SysUserOperateLog> queryList(PageInfoDTO pageInfoDTO) {
        // 构建联查QueryWrapper，LEFT JOIN用户表获取创建人和更新人的用户名
        QueryWrapper queryWrapper = QueryWrapper.create()
                .select(
                        "t1.*",
                        "create_user.MR_NICKNAME as createByName",
                        "update_user.MR_NICKNAME as updateByName"
                )
                .from("SYS_USER_OPERATE_LOG").as("t1")
                .leftJoin("SYS_USER").as("create_user").on("t1.MR_CREATE_BY = create_user.MR_ID")
                .leftJoin("SYS_USER").as("update_user").on("t1.MR_UPDATE_BY = update_user.MR_ID")
                .orderBy("t1.MR_CREATE_TIME desc");

        // 执行分页查询
        return sysUserOperateLogMapper.paginate(pageInfoDTO.getPageNum(), pageInfoDTO.getPageSize(), queryWrapper);
    }
}
