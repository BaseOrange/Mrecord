package com.dcz.mrecord.service.impl;

import cn.hutool.core.util.IdUtil;
import cn.hutool.core.util.StrUtil;
import com.dcz.mrecord.common.ResCode;
import com.dcz.mrecord.common.UserContext;
import com.dcz.mrecord.constant.TempItemTypeConst;
import com.dcz.mrecord.dto.DeleteTempItemDTO;
import com.dcz.mrecord.dto.FinTempItemDTO;
import com.dcz.mrecord.entity.FinBook;
import com.dcz.mrecord.entity.FinMonthItemRecord;
import com.dcz.mrecord.entity.FinTemplateItem;
import com.dcz.mrecord.exception.MrecordException;
import com.dcz.mrecord.mapper.FinBookMapper;
import com.dcz.mrecord.mapper.FinMonthItemRecordMapper;
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
    private FinBookMapper finBookMapper;

    @Resource
    private FinMonthItemRecordMapper finMonthItemRecordMapper;

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

        for (FinTemplateItem finTemplateItem : finTemplateItemList) {
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
        for (FinTemplateItem finTemplateItem : finTemplateItemList) {
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

    /**
     * 删除账本模板项：仅允许删除「尚无任何月份记账记录」的模板项。
     * <p>
     * 与 Rust 端 {@code fin_template_item::delete} 对齐：若存在引用该模板项的
     * 未删除月度明细，抛 {@link ResCode#FIN_ITEM_TEMP_IN_USE}（14306），
     * 历史月度汇总保持不变。
     * <p>
     * 删除前已校验「账簿归属」（补齐 Java 既有 IDOR 缺口）与「无月度明细引用」，
     * 因此这里沿用 {@link #deleteByBookId} 的硬删除（Rust 端为软删除，
     * 但删除前置校验保证无任何引用，两端对前端可观测行为一致：列表不再出现该项）。
     *
     * @param param 删除参数（账簿ID + 模板项ID）
     */
    @Override
    public void deleteFinTemplateItem(DeleteTempItemDTO param) {
        String bookId = param.getBookId();
        String templateItemId = param.getTemplateItemId();
        if (StrUtil.isBlankIfStr(bookId)) {
            throw new MrecordException(ResCode.PARAM_ERROR.getCode(), "账簿ID不能为空");
        }
        if (StrUtil.isBlankIfStr(templateItemId)) {
            throw new MrecordException(ResCode.PARAM_ERROR.getCode(), "账目模板项ID不能为空");
        }

        // 校验账簿归属：账簿必须属于当前登录用户且未删除
        QueryWrapper bookQuery = QueryWrapper.create()
                .eq(FinBook::getId, bookId)
                .eq(FinBook::getUserId, UserContext.getUserId())
                .eq(FinBook::getIsDeleted, 0);
        if (finBookMapper.selectCountByQuery(bookQuery) == 0) {
            throw new MrecordException(ResCode.FIN_BOOK_NOT_FOUND);
        }

        // 模板项必须存在、属于该账簿且未删除
        QueryWrapper itemQuery = QueryWrapper.create()
                .eq(FinTemplateItem::getId, templateItemId)
                .eq(FinTemplateItem::getBookId, bookId)
                .eq(FinTemplateItem::getIsDeleted, 0);
        FinTemplateItem item = finTemplateItemMapper.selectOneByQuery(itemQuery);
        if (item == null) {
            throw new MrecordException(ResCode.FIN_ITEM_TEMP_IS_NOT);
        }

        // 有记账记录则禁止删除（保护历史快照）
        QueryWrapper recordQuery = QueryWrapper.create()
                .eq(FinMonthItemRecord::getTemplateItemId, templateItemId)
                .eq(FinMonthItemRecord::getIsDeleted, 0);
        if (finMonthItemRecordMapper.selectCountByQuery(recordQuery) > 0) {
            throw new MrecordException(ResCode.FIN_ITEM_TEMP_IN_USE);
        }

        finTemplateItemMapper.deleteByQuery(itemQuery);
    }
}
