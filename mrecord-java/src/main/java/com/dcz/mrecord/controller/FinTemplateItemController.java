package com.dcz.mrecord.controller;

import com.dcz.mrecord.common.Result;
import com.dcz.mrecord.dto.FinTempItemDTO;
import com.dcz.mrecord.entity.FinTemplateItem;
import com.dcz.mrecord.service.FinTemplateItemService;
import jakarta.annotation.Resource;
import lombok.extern.slf4j.Slf4j;
import org.springframework.web.bind.annotation.PostMapping;
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
    public Result<List<FinTemplateItem>> createFinTemplateItem(FinTempItemDTO param) {
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
    public Result<List<FinTemplateItem>> updateFinTemplateItem(FinTempItemDTO param) {
        log.info("更新账本模板项[/tempItem/update]请求传参：{}", param);
        List<FinTemplateItem> resList = finTemplateItemService.updateFinTemplateItemList(param);
        return Result.success(resList);
    }

    /**
     * 查询账本模板项列表
     *
     * @param param 查询账本模板项列表参数
     * @return 账本模板项列表
     */
    @PostMapping("/list")
    public Result<List<FinTemplateItem>> listFinTemplateItem(FinTempItemDTO param) {
        log.info("查询账本模板项列表[/tempItem/list]请求传参：{}", param);
        List<FinTemplateItem> resList = finTemplateItemService.selectByFinBookIdExternal(param.getBookId());
        return Result.success(resList);
    }
}
