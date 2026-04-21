package com.dcz.mrecord.service.impl;

import cn.hutool.core.util.IdUtil;
import cn.hutool.core.util.StrUtil;
import com.dcz.mrecord.common.ResCode;
import com.dcz.mrecord.common.UserContext;
import com.dcz.mrecord.dto.QueryFinBookDTO;
import com.dcz.mrecord.entity.FinBook;
import com.dcz.mrecord.exception.MrecordException;
import com.dcz.mrecord.mapper.FinBookMapper;
import com.dcz.mrecord.service.FinBookService;
import com.dcz.mrecord.service.FinMonthItemRecordService;
import com.dcz.mrecord.service.FinTemplateItemService;
import com.mybatisflex.core.paginate.Page;
import com.mybatisflex.core.query.QueryWrapper;
import com.mybatisflex.spring.service.impl.ServiceImpl;
import jakarta.annotation.Resource;
import lombok.extern.slf4j.Slf4j;
import org.springframework.stereotype.Service;

import java.util.Objects;

/**
 * 财务账簿服务实现
 *
 * @author dcz
 * @since 2026/04/09
 */
@Slf4j
@Service
public class FinBookServiceImpl extends ServiceImpl<FinBookMapper, FinBook> implements FinBookService {

    @Resource
    private FinBookMapper finBookMapper;

    @Resource
    private FinTemplateItemService finTemplateItemService;
    @Resource
    private FinMonthItemRecordService finMonthItemRecordService;

    /**
     * 创建账簿
     *
     * @param finBook 账簿
     * @return 账簿
     */
    @Override
    public FinBook createFinBook(FinBook finBook) {
        String userId = UserContext.getUserId();
        finBook.setUserId(userId);

        if (StrUtil.isBlankIfStr(finBook.getBookName())) {
            throw new MrecordException(ResCode.PARAM_ERROR.getCode(), "账簿名称不能为空");
        }

        finBook.setId(IdUtil.simpleUUID());
        finBookMapper.insert(finBook);
        return finBook;
    }

    /**
     * 更新账簿
     *
     * @param finBook 账簿
     * @return 账簿
     */
    @Override
    public FinBook updateFinBook(FinBook finBook) {
        finBookMapper.insertOrUpdateSelective(finBook);
        return finBook;
    }

    /**
     * 删除账簿
     *
     * @param id 账簿ID
     * @return 账簿
     */
    @Override
    public void deleteFinBook(String id) {
        String userId = UserContext.getUserId();
        // 检查账簿是否存在及其归属
        checkUpdateMyFinBook(id, userId);

        // 删除账目数据
        finMonthItemRecordService.deleteByBookId(id);

        // 删除模板数据
        finTemplateItemService.deleteByFinBookId(id);

        // 删除账簿
        finBookMapper.deleteById(id);
    }

    /**
     * 获取我的账簿
     *
     * @param param 查询参数
     * @return 账簿列表
     */
    @Override
    public Page<FinBook> getMyFinBook(QueryFinBookDTO param) {
        Page<FinBook> page = new Page<>(param.getPageNum(), param.getPageSize());

        QueryWrapper qw = QueryWrapper.create();
        String name = param.getName();
        if (StrUtil.isNotBlank(name)) {
            qw.like(FinBook::getBookName, name);
        }
        qw.eq(FinBook::getUserId, UserContext.getUserId());
        return finBookMapper.paginate(page, qw);
    }

    /**
     * 检查更新账簿
     *
     * @param finBookId 账簿ID
     * @param userId    用户ID
     * @return 账簿
     */
    private FinBook checkUpdateMyFinBook(String finBookId, String userId) {
        FinBook dbFinBook = finBookMapper.selectOneById(finBookId);
        // 账簿不存在
        if (dbFinBook == null) {
            throw new MrecordException(ResCode.FIN_BOOK_NOT_FOUND);
        }

        if (!Objects.equals(dbFinBook.getUserId(), userId)) {
            log.info("用户 {} 尝试更新账簿 {}", userId, dbFinBook.getId());
            throw new MrecordException(ResCode.NO_PERMISSION.getCode(), "无该账簿权限，相关操作已记录");
        }

        return dbFinBook;
    }
}
