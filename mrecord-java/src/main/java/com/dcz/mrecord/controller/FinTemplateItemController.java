package com.dcz.mrecord.controller;

import com.dcz.mrecord.service.FinTemplateItemService;
import jakarta.annotation.Resource;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.RestController;

/**
 * 记账模板明细控制器
 *
 * @author dcz
 * @since 2026/04/09
 */
@RestController
@RequestMapping("/templateItem")
public class FinTemplateItemController {
    @Resource
    private FinTemplateItemService finTemplateItemService;
}
