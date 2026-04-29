package com.dcz.mrecord.mapper;

import com.dcz.mrecord.entity.FinMonthRecord;
import com.mybatisflex.core.BaseMapper;
import org.apache.ibatis.annotations.Mapper;
import org.apache.ibatis.annotations.Param;

import java.util.List;
import java.util.Set;

/**
 * 月度财务汇总Mapper
 *
 * @author dcz
 * @since 2026/04/07
 */
@Mapper
public interface FinMonthRecordMapper extends BaseMapper<FinMonthRecord> {

    /**
     * 获取我的账本的最新的财务汇总
     */
    List<FinMonthRecord> getMyBookLastRecord(@Param("bookIdSet") Set<String> bookIdSet);
}
