package com.dcz.mrecord.service;

import com.dcz.mrecord.dto.MonthItemDTO;
import com.dcz.mrecord.entity.FinMonthItemRecord;
import com.mybatisflex.core.service.IService;

import java.util.List;
import java.util.Map;

/**
 * 月度财务账目服务
 *
 * @author dcz
 * @since 2026/04/09
 */
public interface FinMonthItemRecordService extends IService<FinMonthItemRecord> {

    /**
     * 插入月度财务账目
     *
     * @param monthItemDTO 插入月度明细DTO
     */
    List<FinMonthItemRecord> insertMonthItemRecord(MonthItemDTO monthItemDTO);

    /**
     * 删除月度财务账目【仅内部使用，不对外开放接口】
     *
     * @param bookId 账簿ID
     */
    void deleteByBookId(String bookId);

    /**
     * 更新月度财务账目
     *
     * @param monthItemDTO 更新月度明细DTO
     */
    List<FinMonthItemRecord> updateMonthItemRecord(MonthItemDTO monthItemDTO);

    /**
     * 根据账簿和月份查询财务账目
     *
     * @param monthItemDTO 查询条件
     * @return 财务账目列表
     */
    List<FinMonthItemRecord> queryByBookIdAndMonth(MonthItemDTO monthItemDTO);

    /**
     * 根据账簿查询每月财务账目
     *
     * @param monthItemDTO 查询条件
     * @return 每月财务账目列表 Key:yyyyMM Value:FinMonthItemRecord列表
     */
    Map<String, List<FinMonthItemRecord>> queryAllByBookId(MonthItemDTO monthItemDTO);
}
