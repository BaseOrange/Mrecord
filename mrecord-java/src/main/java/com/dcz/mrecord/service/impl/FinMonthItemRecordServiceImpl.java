package com.dcz.mrecord.service.impl;

import cn.hutool.core.util.IdUtil;
import cn.hutool.core.util.StrUtil;
import com.dcz.mrecord.common.ResCode;
import com.dcz.mrecord.dto.MonthItemDTO;
import com.dcz.mrecord.dto.MonthRecordDTO;
import com.dcz.mrecord.entity.FinMonthItemRecord;
import com.dcz.mrecord.entity.FinTemplateItem;
import com.dcz.mrecord.exception.MrecordException;
import com.dcz.mrecord.mapper.FinMonthItemRecordMapper;
import com.dcz.mrecord.service.FinMonthItemRecordService;
import com.dcz.mrecord.service.FinMonthRecordService;
import com.dcz.mrecord.service.FinTemplateItemService;
import com.dcz.mrecord.service.SysBackupMonthItemRecordService;
import com.mybatisflex.core.query.QueryWrapper;
import com.mybatisflex.core.row.Db;
import com.mybatisflex.spring.service.impl.ServiceImpl;
import jakarta.annotation.Resource;
import org.springframework.stereotype.Service;
import org.springframework.transaction.annotation.Transactional;

import java.math.BigDecimal;
import java.util.Collections;
import java.util.List;
import java.util.Map;
import java.util.stream.Collectors;

/**
 * 月度财务账目服务实现
 *
 * @author dcz
 * @since 2026/04/09
 */
@Service
@Transactional(rollbackFor = Exception.class)
public class FinMonthItemRecordServiceImpl extends ServiceImpl<FinMonthItemRecordMapper, FinMonthItemRecord> implements FinMonthItemRecordService {

    @Resource
    private FinMonthItemRecordMapper finMonthItemRecordMapper;

    @Resource
    private FinTemplateItemService finTemplateItemService;
    @Resource
    private FinMonthRecordService finMonthRecordService;
    @Resource
    private SysBackupMonthItemRecordService sysBackupMonthItemRecordService;

    /**
     * 插入月度财务账目
     *
     * @param monthItemDTO 插入月度明细DTO
     */
    @Override
    public List<FinMonthItemRecord> insertMonthItemRecord(MonthItemDTO monthItemDTO) {
        // 检查账簿ID
        checkBookId(monthItemDTO);
        Integer year = monthItemDTO.getYear();
        Integer month = monthItemDTO.getMonth();
        if (year == null || month == null) {
            throw new MrecordException(ResCode.PARAM_ERROR.getCode(), "年份和月份不能为空");
        }
        List<FinMonthItemRecord> finMonthItemRecords = monthItemDTO.getItemList();
        if (finMonthItemRecords == null || finMonthItemRecords.isEmpty()) {
            throw new MrecordException(ResCode.PARAM_ERROR.getCode(), "记账账目列表不能为空");
        }

        // 检查记账账目列表 并 初始化ID、账簿ID、年份、月份
        checkFinItemList(monthItemDTO, true);

        // 批量插入记账账目
        finMonthItemRecordMapper.insertBatch(finMonthItemRecords);

        // 计算月度汇总数据
        finMonthRecordService.calculateFinMonthRecord(monthItemDTO);
        return finMonthItemRecords;
    }

    /**
     * 删除月度财务账目【仅内部使用，不对外开放接口】
     *
     * @param bookId 账簿ID
     */
    @Override
    public void deleteByBookId(String bookId) {
        // 备份
        sysBackupMonthItemRecordService.backup(bookId);
        // 删除
        QueryWrapper qw = new QueryWrapper();
        qw.eq(FinMonthItemRecord::getBookId, bookId);
        finMonthItemRecordMapper.deleteByQuery(qw);
    }

    /**
     * 更新月度财务账目
     *
     * @param monthItemDTO 更新月度明细DTO
     */
    @Override
    public List<FinMonthItemRecord> updateMonthItemRecord(MonthItemDTO monthItemDTO) {
        // 检查账簿ID
        checkBookId(monthItemDTO);

        // 检查记账账目列表
        checkFinItemList(monthItemDTO, false);

        // 批量更新记账账目
        for (FinMonthItemRecord finMonthItemRecord : monthItemDTO.getItemList()) {
            finMonthItemRecordMapper.insertOrUpdateSelective(finMonthItemRecord);
        }

        // 重计算月度汇总数据
        MonthRecordDTO monthRecordDTO = new MonthRecordDTO();
        monthRecordDTO.setBookId(monthItemDTO.getBookId());
        monthRecordDTO.setYear(monthItemDTO.getYear());
        monthRecordDTO.setMonth(monthItemDTO.getMonth());
        monthRecordDTO.setNote(monthItemDTO.getNote());
        finMonthRecordService.recalculateFinMonthRecord(monthRecordDTO);
        return monthItemDTO.getItemList();
    }

    /**
     * 根据账簿和月份查询财务账目
     *
     * @param monthItemDTO 查询条件
     * @return 财务账目列表
     */
    @Override
    public List<FinMonthItemRecord> queryByBookIdAndMonth(MonthItemDTO monthItemDTO) {
        // 检查账簿ID
        String bookId = checkBookId(monthItemDTO);

        // 检查月份、年份
        Integer year = monthItemDTO.getYear();
        Integer month = monthItemDTO.getMonth();
        if (year == null || month == null) {
            throw new MrecordException(ResCode.PARAM_ERROR.getCode(), "年份和月份不能为空");
        }

        QueryWrapper qw = new QueryWrapper();
        qw.eq(FinMonthItemRecord::getBookId, bookId);
        qw.eq(FinMonthItemRecord::getYear, year);
        qw.eq(FinMonthItemRecord::getMonth, month);
        return finMonthItemRecordMapper.selectListByQuery(qw);
    }

    /**
     * 根据账簿查询每月财务账目
     *
     * @param monthItemDTO 查询条件
     * @return 每月财务账目列表 Key:yyyyMM Value:FinMonthItemRecord列表
     */
    @Override
    public Map<String, List<FinMonthItemRecord>> queryAllByBookId(MonthItemDTO monthItemDTO) {
        // 检查账簿ID
        String bookId = checkBookId(monthItemDTO);

        // 构建查询条件
        QueryWrapper qw = new QueryWrapper();
        qw.eq(FinMonthItemRecord::getBookId, bookId);
        List<FinMonthItemRecord> finMonthItemRecords = finMonthItemRecordMapper.selectListByQuery(qw);
        if (finMonthItemRecords == null || finMonthItemRecords.isEmpty()) {
            return Collections.emptyMap();
        }

        // 查询该账簿下的所有模板项，获取排序信息
        Map<String, Integer> sortMap = getSortMap(bookId);

        // 按年月分组，并根据模板项的sort字段排序
        return orderRes(finMonthItemRecords, sortMap);
    }

    /**
     * 校验记账账目列表
     *
     * @param monthItemDTO 插入月度明细DTO
     * @param isInsert     是否为插入
     */
    private void checkFinItemList(MonthItemDTO monthItemDTO, boolean isInsert) {
        List<FinMonthItemRecord> itemList = monthItemDTO.getItemList();

        for (FinMonthItemRecord item : itemList) {
            if (isInsert) {
                item.setId(IdUtil.simpleUUID());
                item.setBookId(monthItemDTO.getBookId());
                item.setYear(monthItemDTO.getYear());
                item.setMonth(monthItemDTO.getMonth());
            }

            String templateItemId = item.getTemplateItemId();
            if (StrUtil.isBlankIfStr(templateItemId)) {
                throw new MrecordException(ResCode.PARAM_ERROR.getCode(), "记账账目ID不能为空");
            }
            BigDecimal itemValue = item.getItemValue();
            if (itemValue == null) {
                throw new MrecordException(ResCode.PARAM_ERROR.getCode(), "记账账目金额不能为空");
            }
            if (itemValue.compareTo(BigDecimal.ZERO) < 0) {
                throw new MrecordException(ResCode.PARAM_ERROR.getCode(), "记账账目金额必须大于等于零");
            }
            String bookId = item.getBookId();
            if (StrUtil.isBlankIfStr(bookId)) {
                throw new MrecordException(ResCode.PARAM_ERROR.getCode(), "账簿ID不能为空");
            }
            Integer year = item.getYear();
            Integer month = item.getMonth();
            if (year == null || month == null) {
                throw new MrecordException(ResCode.PARAM_ERROR.getCode(), "年份和月份不能为空");
            }
        }
    }

    /**
     * 检查账簿ID
     *
     * @param monthItemDTO 账簿ID
     * @return 账簿ID
     */
    private String checkBookId(MonthItemDTO monthItemDTO) {
        String bookId = monthItemDTO.getBookId();
        if (StrUtil.isBlankIfStr(bookId)) {
            throw new MrecordException(ResCode.PARAM_ERROR.getCode(), "账簿ID不能为空");
        }
        return bookId;
    }

    /**
     * 获取排序信息
     *
     * @param bookId 账簿ID
     * @return 排序信息
     */
    private Map<String, Integer> getSortMap(String bookId) {
        List<FinTemplateItem> templateItems = finTemplateItemService.selectByFinBookIdExternal(bookId);
        return templateItems.stream()
                .collect(Collectors.toMap(
                        FinTemplateItem::getId,
                        item -> {
                            try {
                                return Integer.parseInt(item.getSort());
                            } catch (NumberFormatException e) {
                                return Integer.MAX_VALUE;
                            }
                        }
                ));
    }

    /**
     * 按年月分组，并根据模板项的sort字段排序
     *
     * @param finMonthItemRecords 财务账目列表
     * @param sortMap             排序信息
     * @return 按年月分组，并根据模板项的sort字段排序后的财务账目列表
     */
    private Map<String, List<FinMonthItemRecord>> orderRes(List<FinMonthItemRecord> finMonthItemRecords, Map<String, Integer> sortMap) {
        return finMonthItemRecords.stream()
                .collect(Collectors.groupingBy(
                        record -> String.format("%04d%02d", record.getYear(), record.getMonth()),
                        Collectors.collectingAndThen(
                                Collectors.toList(),
                                list -> list.stream()
                                        .sorted((r1, r2) -> {
                                            int sort1 = sortMap.getOrDefault(r1.getTemplateItemId(), Integer.MAX_VALUE);
                                            int sort2 = sortMap.getOrDefault(r2.getTemplateItemId(), Integer.MAX_VALUE);
                                            return Integer.compare(sort1, sort2);
                                        })
                                        .collect(Collectors.toList())
                        )
                ));
    }
}
