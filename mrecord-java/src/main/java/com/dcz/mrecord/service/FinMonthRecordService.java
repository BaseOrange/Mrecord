package com.dcz.mrecord.service;

import com.dcz.mrecord.dto.MonthItemDTO;
import com.dcz.mrecord.dto.MonthRecordDTO;
import com.dcz.mrecord.entity.FinMonthRecord;
import com.mybatisflex.core.service.IService;

import java.util.List;

/**
 * 月度财务汇总服务
 *
 * @author dcz
 * @since 2026/04/09
 */
public interface FinMonthRecordService extends IService<FinMonthRecord> {

    /**
     * 计算月度财务汇总【插入当月数据时，进行计算】
     *
     * @param monthItemDTO 月度项目DTO
     * @return 月度财务汇总
     */
    FinMonthRecord calculateFinMonthRecord(MonthItemDTO monthItemDTO);

    /**
     * 删除月度财务汇总【仅内部调用，不对外开放接口】
     *
     * @param bookId 账簿ID
     */
    void deleteByBookId(String bookId);

    /**
     * 更新后重新计算月度财务汇总【修复某月数据时，进行计算】
     *
     * @param monthRecordDTO 月度汇总DTO
     * @param monthItemDTO
     * @return 月度财务汇总
     */
    FinMonthRecord recalculateFinMonthRecord(MonthRecordDTO monthRecordDTO, MonthItemDTO monthItemDTO);

    /**
     * 获取月度财务汇总
     *
     * @param monthRecordDTO 月度汇总DTO
     * @return 月度财务汇总
     */
    FinMonthRecord getMonthRecord(MonthRecordDTO monthRecordDTO);

    /**
     * 获取全年度财务汇总
     *
     * @param monthRecordDTO 月度汇总DTO
     * @return 全年度财务汇总
     */
    List<FinMonthRecord> getYearRecordList(MonthRecordDTO monthRecordDTO);
}
