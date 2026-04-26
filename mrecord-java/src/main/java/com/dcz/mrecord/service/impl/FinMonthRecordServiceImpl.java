package com.dcz.mrecord.service.impl;

import cn.hutool.core.util.IdUtil;
import cn.hutool.core.util.StrUtil;
import com.dcz.mrecord.common.ResCode;
import com.dcz.mrecord.common.UserContext;
import com.dcz.mrecord.constant.TempItemTypeConst;
import com.dcz.mrecord.dto.MonthItemDTO;
import com.dcz.mrecord.dto.MonthRecordDTO;
import com.dcz.mrecord.entity.FinMonthItemRecord;
import com.dcz.mrecord.entity.FinMonthRecord;
import com.dcz.mrecord.entity.FinTemplateItem;
import com.dcz.mrecord.exception.MrecordException;
import com.dcz.mrecord.mapper.FinMonthRecordMapper;
import com.dcz.mrecord.service.FinBookService;
import com.dcz.mrecord.service.FinMonthRecordService;
import com.dcz.mrecord.service.FinTemplateItemService;
import com.dcz.mrecord.service.SysBackupMonthRecordService;
import com.mybatisflex.core.query.QueryWrapper;
import com.mybatisflex.core.row.Db;
import com.mybatisflex.spring.service.impl.ServiceImpl;
import jakarta.annotation.Resource;
import lombok.extern.slf4j.Slf4j;
import org.springframework.stereotype.Service;
import org.springframework.transaction.annotation.Transactional;

import java.math.BigDecimal;
import java.math.RoundingMode;
import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;
import java.util.Map;
import java.util.stream.Collectors;

/**
 * 月度财务汇总服务实现
 *
 * @author dcz
 * @since 2026/04/09
 */
@Slf4j
@Service
@Transactional(rollbackFor = Exception.class)
public class FinMonthRecordServiceImpl extends ServiceImpl<FinMonthRecordMapper, FinMonthRecord> implements FinMonthRecordService {

    @Resource
    private FinMonthRecordMapper finMonthRecordMapper;

    @Resource
    private FinTemplateItemService finTemplateItemService;
    @Resource
    private SysBackupMonthRecordService sysBackupMonthRecordService;

    /**
     * 计算月度财务汇总【插入当月数据时，进行计算】
     *
     * @param monthItemDTO 月度项目DTO
     * @return 月度财务汇总
     */
    @Override
    public FinMonthRecord calculateFinMonthRecord(MonthItemDTO monthItemDTO) {
        String bookId = monthItemDTO.getBookId();
        if (bookId == null) {
            throw new MrecordException(ResCode.PARAM_ERROR.getCode(), "账簿ID不能为空");
        }
        // 检查年份月份
        Integer year = monthItemDTO.getYear();
        Integer month = monthItemDTO.getMonth();
        if (year == null || month == null) {
            throw new MrecordException(ResCode.PARAM_ERROR.getCode(), "年份或月份不能为空");
        }
        List<FinMonthItemRecord> itemList = monthItemDTO.getItemList();
        if (itemList == null || itemList.isEmpty()) {
            throw new MrecordException(ResCode.PARAM_ERROR.getCode(), "月度账目列表不能为空");
        }

        FinMonthRecord finMonthRecord = new FinMonthRecord();
        finMonthRecord.setId(IdUtil.simpleUUID());
        finMonthRecord.setBookId(bookId);
        finMonthRecord.setYear(year);
        finMonthRecord.setMonth(month);
        // 设置总资产和总负债
        setTotalAssetAndTotalLiability(finMonthRecord, monthItemDTO);
        // 计算净资产
        BigDecimal netAsset = finMonthRecord.getTotalAsset().subtract(finMonthRecord.getTotalLiability()).setScale(2, RoundingMode.HALF_UP);
        finMonthRecord.setNetAsset(netAsset);

        // 获取上个月的财务汇总 并 计算月度环比增长
        FinMonthRecord lastMonthRecord = getLastMonthRecord(bookId, year, month);
        BigDecimal monthOnMonth = getMonthOnMonthVal(lastMonthRecord, finMonthRecord);
        finMonthRecord.setMonthOnMonth(monthOnMonth);

        // 获取去年的财务汇总 并 计算年度同比增长
        FinMonthRecord lastYearRecord = getLastYearRecord(bookId, year, month);
        BigDecimal yearOnYear = getYearOnYearVal(lastYearRecord, finMonthRecord);
        finMonthRecord.setYearOnYear(yearOnYear);

        // 设置备注
        finMonthRecord.setNote(monthItemDTO.getNote());

        finMonthRecordMapper.insert(finMonthRecord);
        return finMonthRecord;
    }

    /**
     * 删除月度财务汇总【仅内部调用，不对外开放接口】
     *
     * @param bookId 账簿ID
     */
    @Override
    public void deleteByBookId(String bookId) {
        // 备份
        sysBackupMonthRecordService.backup(bookId);
        // 删除
        QueryWrapper queryWrapper = QueryWrapper.create();
        queryWrapper.eq(FinMonthRecord::getBookId, bookId);
        finMonthRecordMapper.deleteByQuery(queryWrapper);
    }

    /**
     * 更新后重新计算月度财务汇总【修复某月数据时，进行计算】
     *
     * @param monthRecordDTO 月度汇总DTO
     * @return 月度财务汇总
     */
    @Override
    public FinMonthRecord recalculateFinMonthRecord(MonthRecordDTO monthRecordDTO) {
        // 检查账簿ID和日期
        checkBookIdAndDate(monthRecordDTO);

        String bookId = monthRecordDTO.getBookId();
        Integer year = monthRecordDTO.getYear();
        Integer month = monthRecordDTO.getMonth();

        // 更新集合
        ArrayList<FinMonthRecord> updateList = new ArrayList<>();

        // 获取当前月的财务汇总
        FinMonthRecord currMonthRecord = getMonthRecord(monthRecordDTO);

        // 获取上月的财务汇总，用于计算本月的环比
        boolean updateCurrFlag = false;
        FinMonthRecord lastMonthRecord = getLastMonthRecord(bookId, year, month);
        if (lastMonthRecord != null) {
            BigDecimal currMonthOnMonth = getMonthOnMonthVal(lastMonthRecord, currMonthRecord);
            currMonthRecord.setMonthOnMonth(currMonthOnMonth);
            updateCurrFlag = true;
        }
        // 获取上年当前月的财务汇总，用于计算本月的同比
        FinMonthRecord lastYearRecord = getLastYearRecord(bookId, year, month);
        if (lastYearRecord != null) {
            BigDecimal currYearOnYear = getYearOnYearVal(lastYearRecord, currMonthRecord);
            currMonthRecord.setYearOnYear(currYearOnYear);
            updateCurrFlag = true;
        }
        if (updateCurrFlag) {
            currMonthRecord.setNote(monthRecordDTO.getNote());
            updateList.add(currMonthRecord);
        }

        // 获取下月的财务汇总，用于计算下月的环比
        FinMonthRecord nextMonthRecord = getNextMonthRecord(bookId, year, month);
        if (nextMonthRecord != null) {
            BigDecimal nextMonthOnMonth = getMonthOnMonthVal(currMonthRecord, nextMonthRecord);
            nextMonthRecord.setMonthOnMonth(nextMonthOnMonth);
            updateList.add(nextMonthRecord);
        }
        // 获取明年当前月的财务汇总，用于计算明年当前月的同比
        FinMonthRecord nextYearRecord = getNextYearRecord(bookId, year, month);
        if (nextYearRecord != null) {
            BigDecimal nextYearOnYear = getYearOnYearVal(currMonthRecord, nextYearRecord);
            nextYearRecord.setYearOnYear(nextYearOnYear);
            updateList.add(nextYearRecord);
        }

        // 更新数据库
        Db.updateEntitiesBatch(updateList);
        return currMonthRecord;
    }

    /**
     * 获取月度财务汇总
     *
     * @param monthRecordDTO 月度汇总DTO
     * @return 月度财务汇总
     */
    @Override
    public FinMonthRecord getMonthRecord(MonthRecordDTO monthRecordDTO) {
        // 检查账簿ID和日期
        checkBookIdAndDate(monthRecordDTO);

        QueryWrapper queryWrapper = QueryWrapper.create();
        queryWrapper.eq(FinMonthRecord::getBookId, monthRecordDTO.getBookId());
        queryWrapper.eq(FinMonthRecord::getYear, monthRecordDTO.getYear());
        queryWrapper.eq(FinMonthRecord::getMonth, monthRecordDTO.getMonth());
        return finMonthRecordMapper.selectOneByQuery(queryWrapper);
    }

    /**
     * 获取全年度财务汇总
     *
     * @param monthRecordDTO 月度汇总DTO
     * @return 全年度财务汇总
     */
    @Override
    public List<FinMonthRecord> getYearRecordList(MonthRecordDTO monthRecordDTO) {
        String bookId = monthRecordDTO.getBookId();
        if (StrUtil.isBlankIfStr(bookId)) {
            throw new MrecordException(ResCode.PARAM_ERROR.getCode(), "账簿ID不能为空");
        }

        QueryWrapper queryWrapper = QueryWrapper.create();
        queryWrapper.eq(FinMonthRecord::getBookId, bookId);
        Integer year = monthRecordDTO.getYear();
        if (year != null){
            queryWrapper.eq(FinMonthRecord::getYear, year);
        }
        return finMonthRecordMapper.selectListByQuery(queryWrapper);
    }

    /**
     * 检查账簿ID和日期
     *
     * @param monthRecordDTO 月度汇总DTO
     */
    private void checkBookIdAndDate(MonthRecordDTO monthRecordDTO) {
        String bookId = monthRecordDTO.getBookId();
        if (StrUtil.isBlankIfStr(bookId)) {
            throw new MrecordException(ResCode.PARAM_ERROR.getCode(), "账簿ID不能为空");
        }
        Integer year = monthRecordDTO.getYear();
        Integer month = monthRecordDTO.getMonth();
        if (year == null || month == null) {
            throw new MrecordException(ResCode.PARAM_ERROR.getCode(), "年份或月份不能为空");
        }
    }

    /**
     * 获取去年的财务汇总
     *
     * @param bookId    账簿ID
     * @param currYear  当前年份
     * @param currMonth 当前月份
     * @return 去年的财务汇总
     */
    private FinMonthRecord getLastYearRecord(String bookId, Integer currYear, Integer currMonth) {
        QueryWrapper queryWrapper = QueryWrapper.create();
        queryWrapper.eq(FinMonthRecord::getBookId, bookId);
        queryWrapper.eq(FinMonthRecord::getYear, currYear);
        queryWrapper.eq(FinMonthRecord::getMonth, currMonth);
        FinMonthRecord finMonthRecord = finMonthRecordMapper.selectOneByQuery(queryWrapper);
        if (finMonthRecord == null) {
            log.info("用户 {} 尝试获取去年的财务汇总，但去年的账簿中不存在该月份的数据", UserContext.getUserId());
        }
        return finMonthRecord;
    }

    /**
     * 获取上个月的财务汇总
     *
     * @param bookId    账簿ID
     * @param currYear  当前年份
     * @param currMonth 当前月份
     * @return 上个月的财务汇总
     */
    private FinMonthRecord getLastMonthRecord(String bookId, Integer currYear, Integer currMonth) {
        // 判断是否为一月份
        boolean isJanuary = currMonth == 1;

        // 查询去年12月数据
        if (isJanuary) {
            currYear = currYear - 1;
            currMonth = 12;
        } else {
            currMonth = currMonth - 1;
        }

        QueryWrapper queryWrapper = QueryWrapper.create();
        queryWrapper.eq(FinMonthRecord::getBookId, bookId);
        queryWrapper.eq(FinMonthRecord::getYear, currYear);
        queryWrapper.eq(FinMonthRecord::getMonth, currMonth);
        FinMonthRecord finMonthRecord = finMonthRecordMapper.selectOneByQuery(queryWrapper);
        if (finMonthRecord == null) {
            log.info("用户 {} 尝试获取上月{}的财务汇总，但账簿中不存在该月份的数据", UserContext.getUserId(), currMonth);
        }
        return finMonthRecord;
    }

    /**
     * 获取明年的财务汇总
     *
     * @param bookId    账簿ID
     * @param currYear  当前年份
     * @param currMonth 当前月份
     * @return 明年的财务汇总
     */
    private FinMonthRecord getNextYearRecord(String bookId, Integer currYear, Integer currMonth) {
        QueryWrapper queryWrapper = QueryWrapper.create();
        queryWrapper.eq(FinMonthRecord::getBookId, bookId);
        queryWrapper.eq(FinMonthRecord::getYear, currYear + 1);
        queryWrapper.eq(FinMonthRecord::getMonth, currMonth);
        FinMonthRecord finMonthRecord = finMonthRecordMapper.selectOneByQuery(queryWrapper);
        if (finMonthRecord == null) {
            log.info("用户 {} 尝试获取明年的财务汇总，但明年的账簿中不存在该月份的数据", UserContext.getUserId());
        }
        return finMonthRecord;
    }

    /**
     * 获取下个月的财务汇总
     *
     * @param bookId    账簿ID
     * @param currYear  当前年份
     * @param currMonth 当前月份
     * @return 下个月的财务汇总
     */
    private FinMonthRecord getNextMonthRecord(String bookId, Integer currYear, Integer currMonth) {
        // 判断是否为12月份
        boolean isDecember = currMonth == 12;

        // 查询明年1月数据
        if (isDecember) {
            currYear = currYear + 1;
            currMonth = 1;
        } else {
            currMonth = currMonth + 1;
        }

        QueryWrapper queryWrapper = QueryWrapper.create();
        queryWrapper.eq(FinMonthRecord::getBookId, bookId);
        queryWrapper.eq(FinMonthRecord::getYear, currYear);
        queryWrapper.eq(FinMonthRecord::getMonth, currMonth);
        FinMonthRecord finMonthRecord = finMonthRecordMapper.selectOneByQuery(queryWrapper);
        if (finMonthRecord == null) {
            log.info("用户 {} 尝试获取下月{}的财务汇总，但账簿中不存在该月份的数据", UserContext.getUserId(), currMonth);
        }
        return finMonthRecord;
    }

    /**
     * 设置总资产和总负债
     *
     * @param finMonthRecord 月度财务汇总
     * @param monthItemDTO   月度项目DTO
     */
    private void setTotalAssetAndTotalLiability(FinMonthRecord finMonthRecord, MonthItemDTO monthItemDTO) {
        List<FinTemplateItem> finTemplateItems = finTemplateItemService.selectByFinBookIdExternal(monthItemDTO.getBookId());
        Map<String, Integer> tempTypeMap = finTemplateItems.stream().collect(Collectors.toMap(
                FinTemplateItem::getId,
                FinTemplateItem::getItemType,
                (existing, replacement) -> replacement,
                HashMap::new
        ));

        BigDecimal totalAsset = new BigDecimal(0);
        BigDecimal totalLiability = new BigDecimal(0);

        List<FinMonthItemRecord> itemList = monthItemDTO.getItemList();
        for (FinMonthItemRecord item : itemList) {
            String tempId = item.getTemplateItemId();
            Integer itemType = tempTypeMap.get(tempId);
            BigDecimal itemValue = item.getItemValue();
            if (itemType != null) {
                if (TempItemTypeConst.ASSET.intValue() == itemType) {
                    totalAsset = totalAsset.add(itemValue);
                } else if (TempItemTypeConst.LIABILITY.intValue() == itemType) {
                    totalLiability = totalLiability.add(itemValue);
                }
            }
        }

        finMonthRecord.setTotalAsset(totalAsset);
        finMonthRecord.setTotalLiability(totalLiability);
    }

    /**
     * 获取年度同比增长
     *
     * @param lastYearRecord  去年财务汇总
     * @param currMonthRecord 今年本月财务汇总
     * @return 年度同比增长
     */
    private BigDecimal getYearOnYearVal(FinMonthRecord lastYearRecord, FinMonthRecord currMonthRecord) {
        if (lastYearRecord != null && lastYearRecord.getNetAsset() != null) {
            BigDecimal lastYearNetAsset = lastYearRecord.getNetAsset();
            BigDecimal netAsset = currMonthRecord.getNetAsset();
            return netAsset
                    .divide(lastYearNetAsset, 4, RoundingMode.HALF_UP)
                    .subtract(BigDecimal.ONE)
                    .multiply(BigDecimal.valueOf(100))
                    .setScale(2, RoundingMode.HALF_UP);
        }
        return BigDecimal.ZERO;
    }

    /**
     * 获取月度环比增长
     *
     * @param lastMonthRecord 上月财务汇总
     * @param currMonthRecord 今年本月财务汇总
     * @return 月度环比增长
     */
    private BigDecimal getMonthOnMonthVal(FinMonthRecord lastMonthRecord, FinMonthRecord currMonthRecord) {
        if (lastMonthRecord != null && lastMonthRecord.getNetAsset() != null) {
            BigDecimal lastMonthNetAsset = lastMonthRecord.getNetAsset();
            BigDecimal netAsset = currMonthRecord.getNetAsset();
            // 本月除以上月 减去1再乘以100 得到环比增长
            return netAsset
                    .divide(lastMonthNetAsset, 4, RoundingMode.HALF_UP)
                    .subtract(BigDecimal.ONE)
                    .multiply(BigDecimal.valueOf(100))
                    .setScale(2, RoundingMode.HALF_UP);
        }
        return BigDecimal.ZERO;
    }
}
