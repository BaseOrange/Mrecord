package com.dcz.mrecord.service.impl;

import cn.hutool.core.util.IdUtil;
import cn.hutool.core.util.StrUtil;
import com.dcz.mrecord.common.ResCode;
import com.dcz.mrecord.common.UserContext;
import com.dcz.mrecord.constant.TempItemTypeConst;
import com.dcz.mrecord.dto.FinTempItemDTO;
import com.dcz.mrecord.entity.FinTemplateItem;
import com.dcz.mrecord.exception.MrecordException;
import com.dcz.mrecord.mapper.FinTemplateItemMapper;
import com.dcz.mrecord.service.FinTemplateItemService;
import com.dcz.mrecord.service.SysBackupTemplateItemService;
import com.mybatisflex.core.query.QueryWrapper;
import com.mybatisflex.core.row.Db;
import com.mybatisflex.spring.service.impl.ServiceImpl;
import jakarta.annotation.Resource;
import org.springframework.stereotype.Service;
import org.springframework.transaction.annotation.Transactional;

import java.util.ArrayList;
import java.util.Collections;
import java.util.List;
import java.util.Optional;

/**
 * 记账模板明细服务实现
 *
 * @author dcz
 * @since 2026/04/09
 */
@Service
@Transactional(rollbackFor = Exception.class)
public class FinTemplateItemServiceImpl extends ServiceImpl<FinTemplateItemMapper, FinTemplateItem> implements FinTemplateItemService {

    @Resource
    private FinTemplateItemMapper finTemplateItemMapper;

    @Resource
    private SysBackupTemplateItemService sysBackupTemplateItemService;

    /**
     * 创建记账模板
     *
     * @param param 创建记账模板参数
     * @return 记账模板明细列表
     */
    @Override
    public List<FinTemplateItem> ceateFinTemplateItemList(FinTempItemDTO param) {
        String finBookId = param.getBookId();
        List<FinTemplateItem> finTemplateItemList = param.getItemList();

        if (finTemplateItemList == null || finTemplateItemList.isEmpty()) {
            throw new MrecordException(ResCode.FIN_ITEM_TEMP_IS_NOT);
        }

        for (int i = 0; i < finTemplateItemList.size(); i++) {
            FinTemplateItem finTemplateItem = finTemplateItemList.get(i);
            finTemplateItem.setId(IdUtil.simpleUUID());
            finTemplateItem.setBookId(finBookId);

            String itemName = finTemplateItem.getItemName();
            if (StrUtil.isBlankIfStr(itemName)) {
                throw new MrecordException(ResCode.FIN_ITEM_TEMP_NAME_IS_NOT);
            }
            Integer itemType = finTemplateItem.getItemType();
            if (itemType == null) {
                throw new MrecordException(ResCode.FIN_ITEM_TEMP_TYPE_IS_NOT);
            }
            int type = itemType;
            if (type != TempItemTypeConst.LIABILITY && type != TempItemTypeConst.ASSET && type != TempItemTypeConst.ONLY_RECORD) {
                throw new MrecordException(ResCode.FIN_ITEM_TEMP_TYPE_ERROR);
            }
            finTemplateItem.setSort(String.valueOf(i));
        }

        finTemplateItemMapper.insertBatch(finTemplateItemList);
        return finTemplateItemList;
    }

    /**
     * 根据账簿ID删除记账模板明细
     *
     * @param finBookId 账簿ID
     */
    @Override
    public void deleteByBookId(String finBookId) {
        // 备份
        sysBackupTemplateItemService.backup(finBookId);
        // 这里不再校验，前置方法删除账簿已经进行校验
        QueryWrapper queryWrapper = QueryWrapper.create();
        queryWrapper.eq(FinTemplateItem::getBookId, finBookId);
        finTemplateItemMapper.deleteByQuery(queryWrapper);
    }

    /**
     * 更新记账模板明细
     *
     * @param param 修改记账模板参数
     * @return 记账模板明细列表
     */
    @Override
    public List<FinTemplateItem> updateFinTemplateItemList(FinTempItemDTO param) {
        String finBookId = param.getBookId();
        List<FinTemplateItem> finTemplateItemList = param.getItemList();

        List<FinTemplateItem> dbList = selectByFinBookId(finBookId);

        List<FinTemplateItem> updateList = new ArrayList<>();
        List<FinTemplateItem> insertList = new ArrayList<>();
        for (int i = 0; i < finTemplateItemList.size(); i++) {
            FinTemplateItem finTemplateItem = finTemplateItemList.get(i);
            finTemplateItem.setSort(String.valueOf(i));

            // Id为空，新模板项
            String id = finTemplateItem.getId();
            if (StrUtil.isBlankIfStr(id)) {
                finTemplateItem.setId(IdUtil.simpleUUID());
                insertList.add(finTemplateItem);
                continue;
            }
            // 判断是否被删除、或者类型是否被修改
            Optional<FinTemplateItem> any = dbList.stream().filter(item -> id.equals(item.getId())).findAny();
            if (any.isEmpty()) {
                throw new MrecordException(ResCode.FIN_ITEM_TEMP_UPDATE_ERROR);
            }
            FinTemplateItem dbItem = any.get();
            if (!dbItem.getItemType().equals(finTemplateItem.getItemType())) {
                throw new MrecordException(ResCode.FIN_ITEM_TEMP_UPDATE_ERROR);
            }
            updateList.add(finTemplateItem);
        }

        // 插入新模板项
        if (!insertList.isEmpty()) {
            finTemplateItemMapper.insertBatch(insertList);
        }
        // 更新模板
        if (!updateList.isEmpty()) {
            Db.updateEntitiesBatch(updateList, 1000);
        }

        return finTemplateItemList;
    }

    /**
     * 复制记账模板
     *
     * @param param 复制月度明细DTO
     */
    @Override
    public List<FinTemplateItem> copyTemplateItem(FinTempItemDTO param) {
        String oldBookId = param.getOldBookId();
        if (StrUtil.isBlankIfStr(oldBookId)) {
            throw new MrecordException(ResCode.PARAM_ERROR.getCode(), "原账簿ID不能为空");
        }
        String newBookId = param.getBookId();
        if (StrUtil.isBlankIfStr(newBookId)) {
            throw new MrecordException(ResCode.PARAM_ERROR.getCode(), "新账簿ID不能为空");
        }

        List<FinTemplateItem> finTemplateItems = selectByFinBookId(oldBookId);
        if (finTemplateItems == null || finTemplateItems.isEmpty()) {
            return Collections.emptyList();
        }

        for (FinTemplateItem finTemplateItem : finTemplateItems) {
            finTemplateItem.setId(IdUtil.simpleUUID());
            finTemplateItem.setBookId(newBookId);
        }
        finTemplateItemMapper.insertBatch(finTemplateItems);
        return finTemplateItems;
    }

    /**
     * 根据账簿ID查询记账模板明细【对外，校验账本权限】
     *
     * @param finBookId 账簿ID
     * @return 记账模板明细列表
     */
    @Override
    public List<FinTemplateItem> selectByFinBookIdExternal(String finBookId) {
        List<FinTemplateItem> finTemplateItems = finTemplateItemMapper.selectListByBookIdAndUserId(finBookId, UserContext.getUserId());
        if (finTemplateItems == null || finTemplateItems.isEmpty()) {
            throw new MrecordException(ResCode.FIN_ITEM_TEMP_IS_NOT);
        }
        return finTemplateItems;
    }

    /**
     * 根据账簿ID查询记账模板明细
     *
     * @param finBookId 账簿ID
     * @return 记账模板明细列表
     */
    private List<FinTemplateItem> selectByFinBookId(String finBookId) {
        QueryWrapper queryWrapper = QueryWrapper.create();
        queryWrapper.eq(FinTemplateItem::getBookId, finBookId);
        return finTemplateItemMapper.selectListByQuery(queryWrapper);
    }
}
