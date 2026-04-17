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
        return sysUserOperateLogMapper.paginate(pageInfoDTO.getPageNum(), pageInfoDTO.getPageSize(), QueryWrapper.create());
    }
}
