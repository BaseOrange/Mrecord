package com.dcz.mrecord.mapper;

import com.dcz.mrecord.entity.FinTemplateItem;
import com.mybatisflex.core.BaseMapper;
import org.apache.ibatis.annotations.Mapper;
import org.apache.ibatis.annotations.Param;

import java.util.List;

/**
 * 记账模板明细Mapper
 *
 * @author dcz
 * @since 2026/04/07
 */
@Mapper
public interface FinTemplateItemMapper extends BaseMapper<FinTemplateItem> {
    /**
     * 根据账本id和用户id查询记账模板明细列表
     *
     * @param finBookId 账本id
     * @param userId    用户id
     * @return 记账模板明细列表
     */
    List<FinTemplateItem> selectListByBookIdAndUserId(@Param("finBookId") String finBookId, @Param("userId") String userId);
}
