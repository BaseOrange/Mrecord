package com.dcz.mrecord.controller;

import com.dcz.mrecord.common.Result;
import com.dcz.mrecord.dto.DeleteTempItemDTO;
import com.dcz.mrecord.dto.FinTempItemDTO;
import com.dcz.mrecord.entity.FinTemplateItem;
import com.dcz.mrecord.service.FinTemplateItemService;
import jakarta.annotation.Resource;
import lombok.extern.slf4j.Slf4j;
import org.springframework.web.bind.annotation.PostMapping;
import org.springframework.web.bind.annotation.RequestBody;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.RestController;

import java.util.List;

/**
 * 记账模板明细控制器
 *
 * @author dcz
 * @since 2026/04/09
 */
@Slf4j
@RestController
@RequestMapping("/tempItem")
public class FinTemplateItemController {
    @Resource
    private FinTemplateItemService finTemplateItemService;

    /**
     * 创建账本模板项
     *
     * @param param 创建账本模板项参数
     * @return 账本模板项列表
     */
    @PostMapping("/create")
    public Result<List<FinTemplateItem>> createFinTemplateItem(@RequestBody FinTempItemDTO param) {
        log.info("创建账本模板项[/tempItem/create]请求传参：{}", param);
        List<FinTemplateItem> resList = finTemplateItemService.ceateFinTemplateItemList(param);
        return Result.success(resList);
    }

    /**
     * 更新账本模板项
     *
     * @param param 更新账本模板项参数
     * @return 账本模板项列表
     */
    @PostMapping("/update")
    public Result<List<FinTemplateItem>> updateFinTemplateItem(@RequestBody FinTempItemDTO param) {
        log.info("更新账本模板项[/tempItem/update]请求传参：{}", param);
        List<FinTemplateItem> resList = finTemplateItemService.updateFinTemplateItemList(param);
        return Result.success(resList);
    }

    /**
     * 复制账本模板项
     *
     * @param param 复制账本模板项参数
     * @return 账本模板项列表
     */
    @PostMapping("/copy")
    public Result<List<FinTemplateItem>> copyFinTemplateItem(@RequestBody FinTempItemDTO param) {
        log.info("复制账本模板项[/tempItem/copy]请求传参：{}", param);
        List<FinTemplateItem> resList = finTemplateItemService.copyTemplateItem(param);
        return Result.success(resList);
    }

    /**
     * 查询账本模板项列表
     *
     * @param param 查询账本模板项列表参数
     * @return 账本模板项列表
     */
    @PostMapping("/list")
    public Result<List<FinTemplateItem>> listFinTemplateItem(@RequestBody FinTempItemDTO param) {
        log.info("查询账本模板项列表[/tempItem/list]请求传参：{}", param);
        List<FinTemplateItem> resList = finTemplateItemService.selectByFinBookIdExternal(param.getBookId());
        return Result.success(resList);
    }

    /**
     * 删除账本模板项
     * <p>
     * 仅允许删除尚无记账记录的模板项；已有记录的科目返回 14306，
     * 以保护历史月度快照。
     *
     * @param param 删除账本模板项参数（账簿ID + 模板项ID）
     * @return 无
     */
    @PostMapping("/delete")
    public Result<Void> deleteFinTemplateItem(@RequestBody DeleteTempItemDTO param) {
        log.info("删除账本模板项[/tempItem/delete]请求传参：{}", param);
        finTemplateItemService.deleteFinTemplateItem(param);
        return Result.success();
    }
}
