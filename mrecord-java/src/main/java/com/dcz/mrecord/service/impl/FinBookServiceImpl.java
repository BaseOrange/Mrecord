package com.dcz.mrecord.service.impl;

import cn.hutool.core.util.IdUtil;
import cn.hutool.core.util.StrUtil;
import com.dcz.mrecord.common.ResCode;
import com.dcz.mrecord.common.UserContext;
import com.dcz.mrecord.constant.FinbookTypeConst;
import com.dcz.mrecord.dto.QueryFinBookDTO;
import com.dcz.mrecord.entity.FinBook;
import com.dcz.mrecord.exception.MrecordException;
import com.dcz.mrecord.mapper.FinBookMapper;
import com.dcz.mrecord.service.FinBookService;
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

        String bookType = finBook.getBookType();
        if (StrUtil.isBlankIfStr(bookType)) {
            throw new MrecordException(ResCode.PARAM_ERROR.getCode(), "账簿类型不能为空");
        }
        if (!FinbookTypeConst.YEARLY.equals(bookType) && !FinbookTypeConst.CATEGORY.equals(bookType)) {
            throw new MrecordException(ResCode.PARAM_ERROR.getCode(), "账簿类型错误");
        }
        if (StrUtil.isBlankIfStr(finBook.getYear())) {
            throw new MrecordException(ResCode.PARAM_ERROR.getCode(), "账簿年份不能为空");
        }
        if (yearlyBookRepeatCheck(userId, finBook.getYear())) {
            throw new MrecordException(ResCode.PARAM_ERROR.getCode(), "账簿已存在");
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
        String userId = UserContext.getUserId();
        FinBook dbFinBook = checkUpdateMyFinBook(finBook.getId(), userId);
        if (!Objects.equals(dbFinBook.getBookType(), finBook.getBookType())) {
            log.info("用户 {} 尝试更新账簿 {} 的账簿类型", userId, finBook.getId());
            throw new MrecordException(ResCode.FIN_BOOK_TYPE_UPDATE);
        }
        if (!Objects.equals(dbFinBook.getYear(), finBook.getYear())) {
            log.info("用户 {} 尝试更新账簿 {} 的账簿年份", userId, finBook.getId());
            throw new MrecordException(ResCode.FIN_BOOK_YEAR_UPDATE);
        }

        finBookMapper.update(finBook);
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

        // TODO 删除账目数据

        // TODO 删除模板数据

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
        String type = param.getType();
        if (StrUtil.isNotBlank(type)) {
            qw.eq(FinBook::getBookType, type);
        }
        String year = param.getYear();
        if (StrUtil.isNotBlank(year)) {
            qw.eq(FinBook::getYear, year);
        }
        qw.eq(FinBook::getUserId, UserContext.getUserId());
        return finBookMapper.paginate(page, qw);
    }

    /**
     * 年度账簿重复检查
     *
     * @param userId 用户ID
     * @param year   账簿年份
     * @return 是否重复
     */
    private boolean yearlyBookRepeatCheck(String userId, String year) {
        QueryWrapper qw = QueryWrapper.create();
        qw.eq(FinBook::getUserId, userId).eq(FinBook::getBookType, FinbookTypeConst.YEARLY).eq(FinBook::getYear, year);
        return finBookMapper.selectCountByQuery(qw) > 0;
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
