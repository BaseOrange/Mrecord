package com.dcz.mrecord.service;

import com.dcz.mrecord.dto.FinTempItemDTO;
import com.dcz.mrecord.entity.FinTemplateItem;
import com.mybatisflex.core.service.IService;

import java.util.List;

/**
 * 记账模板明细服务
 *
 * @author dcz
 * @since 2026/04/09
 */
public interface FinTemplateItemService extends IService<FinTemplateItem> {
    /**
     * 创建记账模板
     *
     * @param param 创建记账模板参数
     * @return 记账模板明细列表
     */
    List<FinTemplateItem> ceateFinTemplateItemList(FinTempItemDTO param);

    /**
     * 根据账簿ID删除记账模板明细
     *
     * @param finBookId 账簿ID
     */
    void deleteByFinBookId(String finBookId);

    /**
     * 更新记账模板明细
     *
     * @param param 修改记账模板参数
     * @return 记账模板明细列表
     */
    List<FinTemplateItem> updateFinTemplateItemList(FinTempItemDTO param);

    /**
     * 复制记账模板
     *
     * @param param 复制月度明细DTO
     */
    List<FinTemplateItem> copyTemplateItem(FinTempItemDTO param);

    /**
     * 根据账簿ID查询记账模板明细【对外，校验账本权限】
     *
     * @param finBookId 账簿ID
     * @return 记账模板明细列表
     */
    List<FinTemplateItem> selectByFinBookIdExternal(String finBookId);
}
