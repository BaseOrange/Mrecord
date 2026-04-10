package com.dcz.mrecord.controller;

import com.dcz.mrecord.service.FinBookService;
import jakarta.annotation.Resource;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.RestController;

/**
 * 财务账簿控制器
 *
 * @author dcz
 * @since 2026/04/09
 */
@RestController
@RequestMapping("/book")
public class FinBookController {
    @Resource
    private FinBookService finBookService;
}
