package com.dcz.mrecord.service.impl;

import cn.hutool.core.bean.BeanUtil;
import cn.hutool.core.date.DateTime;
import cn.hutool.core.date.DateUtil;
import cn.hutool.core.util.IdUtil;
import cn.hutool.core.util.StrUtil;
import com.dcz.mrecord.common.ResCode;
import com.dcz.mrecord.common.UserContext;
import com.dcz.mrecord.dto.DataStatisticsDTO;
import com.dcz.mrecord.dto.FinBookRecordDTO;
import com.dcz.mrecord.dto.IdDto;
import com.dcz.mrecord.dto.QueryFinBookDTO;
import com.dcz.mrecord.entity.FinBook;
import com.dcz.mrecord.entity.FinMonthRecord;
import com.dcz.mrecord.exception.MrecordException;
import com.dcz.mrecord.mapper.FinBookMapper;
import com.dcz.mrecord.service.FinBookService;
import com.dcz.mrecord.service.FinMonthItemRecordService;
import com.dcz.mrecord.service.FinMonthRecordService;
import com.dcz.mrecord.service.FinTemplateItemService;
import com.dcz.mrecord.service.SysBackupBookService;
import com.mybatisflex.core.paginate.Page;
import com.mybatisflex.core.query.QueryWrapper;
import com.mybatisflex.spring.service.impl.ServiceImpl;
import jakarta.annotation.Resource;
import lombok.extern.slf4j.Slf4j;
import org.springframework.stereotype.Service;
import org.springframework.transaction.annotation.Transactional;

import java.util.ArrayList;
import java.util.List;
import java.util.Objects;
import java.util.Optional;
import java.util.Set;
import java.util.stream.Collectors;

/**
 * 财务账簿服务实现
 *
 * @author dcz
 * @since 2026/04/09
 */
@Slf4j
@Service
@Transactional(rollbackFor = Exception.class)
public class FinBookServiceImpl extends ServiceImpl<FinBookMapper, FinBook> implements FinBookService {

    @Resource
    private FinBookMapper finBookMapper;

    @Resource
    private FinTemplateItemService finTemplateItemService;
    @Resource
    private FinMonthItemRecordService finMonthItemRecordService;
    @Resource
    private FinMonthRecordService finMonthRecordService;
    @Resource
    private SysBackupBookService sysBackupBookService;

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
     * @param idDto 账簿ID
     * @return 账簿
     */
    @Override
    public void deleteFinBook(IdDto idDto) {
        String userId = UserContext.getUserId();
        // 检查账簿是否存在及其归属
        String id = idDto.getId();
        checkUpdateMyFinBook(id, userId);

        // 删除账目数据
        finMonthItemRecordService.deleteByBookId(id);

        // 删除月度汇总数据
        finMonthRecordService.deleteByBookId(id);

        // 删除模板数据
        finTemplateItemService.deleteByBookId(id);

        // 删除账簿
        sysBackupBookService.backup(id);
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
     * 获取所有账户的统计数据
     *
     * @return 统计数据DTO
     */
    @Override
    public DataStatisticsDTO<FinBookRecordDTO> getMyDataStatistics() {
        QueryWrapper qw = QueryWrapper.create();
        qw.eq(FinBook::getUserId, UserContext.getUserId());
        List<FinBook> finBooks = finBookMapper.selectListByQuery(qw);

        DataStatisticsDTO<FinBookRecordDTO> dataStatisticsDTO = new DataStatisticsDTO<>();
        if (finBooks == null || finBooks.isEmpty()) {
            return dataStatisticsDTO;
        }

        // 获取账目数据
        Set<String> bookIdSet = finBooks.stream().map(FinBook::getId).collect(Collectors.toSet());
        List<FinMonthRecord> finMonthRecordList = finMonthRecordService.getMyBookLastRecord(bookIdSet);

        // 对象转换
        List<FinBookRecordDTO> resList = new ArrayList<>();
        for (FinMonthRecord finMonthRecord : finMonthRecordList) {
            FinBookRecordDTO finBookRecordDTO = new FinBookRecordDTO();
            BeanUtil.copyProperties(finMonthRecord, finBookRecordDTO);
            Optional<FinBook> any = finBooks.stream().filter(book -> book.getId().equals(finMonthRecord.getBookId())).findAny();
            if (any.isEmpty()) {
                log.warn("账目数据 {} 所属账簿不存在", finMonthRecord.getId());
                continue;
            }
            FinBook finBook = any.get();
            finBookRecordDTO.setBookId(finBook.getId());
            finBookRecordDTO.setBookName(finBook.getBookName());
            resList.add(finBookRecordDTO);
        }
        dataStatisticsDTO.setRecordList(resList);
        return dataStatisticsDTO;
    }

    /**
     * 获取指定账户的详细统计数据
     *
     * @param idDTO 账簿ID
     * @return 统计数据DTO
     */
    @Override
    public DataStatisticsDTO<FinMonthRecord> getBookDetailedStatistics(IdDto idDTO) {
        String id = idDTO.getId();
        if (StrUtil.isBlankIfStr(id)) {
            throw new MrecordException(ResCode.PARAM_ERROR.getCode(), "账簿ID不能为空");
        }
        checkUpdateMyFinBook(id, UserContext.getUserId());

        // 设置开始结束的年份和月份
        DataStatisticsDTO<FinMonthRecord> dataStatisticsDTO = new DataStatisticsDTO<>();
        DateTime date = DateUtil.date();
        dataStatisticsDTO.setStartYearMonth(DateUtil.format(DateUtil.offsetYear(date, -1), "yyyyMM"));
        dataStatisticsDTO.setEndYearMonth(DateUtil.format(date, "yyyyMM"));

        List<FinMonthRecord> finMonthRecordList = finMonthRecordService.getBookOneYearRecord(id, dataStatisticsDTO);
        dataStatisticsDTO.setRecordList(finMonthRecordList);
        return dataStatisticsDTO;
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
