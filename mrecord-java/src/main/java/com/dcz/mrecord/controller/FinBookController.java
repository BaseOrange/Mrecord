package com.dcz.mrecord.controller;

import com.dcz.mrecord.common.Result;
import com.dcz.mrecord.dto.DataStatisticsDTO;
import com.dcz.mrecord.dto.FinBookRecordDTO;
import com.dcz.mrecord.dto.IdDto;
import com.dcz.mrecord.dto.QueryFinBookDTO;
import com.dcz.mrecord.entity.FinBook;
import com.dcz.mrecord.entity.FinMonthRecord;
import com.dcz.mrecord.service.FinBookService;
import com.mybatisflex.core.paginate.Page;
import jakarta.annotation.Resource;
import lombok.extern.slf4j.Slf4j;
import org.springframework.web.bind.annotation.PostMapping;
import org.springframework.web.bind.annotation.RequestBody;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.RestController;

/**
 * 财务账簿控制器
 *
 * @author dcz
 * @since 2026/04/09
 */
@Slf4j
@RestController
@RequestMapping("/book")
public class FinBookController {

    @Resource
    private FinBookService finBookService;

    /**
     * 创建账簿
     *
     * @param params 账簿参数
     * @return 账簿
     */
    @PostMapping("/create")
    public Result<FinBook> create(@RequestBody FinBook params) {
        log.info("创建账簿[/book/create]请求传参：{}", params);
        return Result.success(finBookService.createFinBook(params));
    }

    /**
     * 更新账簿
     *
     * @param params 账簿参数
     * @return 账簿
     */
    @PostMapping("/update")
    public Result<FinBook> update(@RequestBody FinBook params) {
        log.info("更新账簿[/book/update]请求传参：{}", params);
        return Result.success(finBookService.updateFinBook(params));
    }

    /**
     * 删除账簿
     *
     * @param id 账簿ID
     * @return 删除结果
     */
    @PostMapping("/delete")
    public Result<String> delete(@RequestBody IdDto id) {
        log.info("删除账簿[/book/delete]请求传参：{}", id);
        finBookService.deleteFinBook(id);
        return Result.success();
    }

    /**
     * 获取账簿列表
     *
     * @param params 账簿参数
     * @return 账簿列表
     */
    @PostMapping("/list")
    public Result<Page<FinBook>> list(@RequestBody QueryFinBookDTO params) {
        log.info("获取账簿列表[/book/list]请求传参：{}", params);
        return Result.success(finBookService.getMyFinBook(params));
    }

    /**
     * 获取所有账户的统计数据
     *
     * @return 统计数据DTO
     */
    @PostMapping("/getMyDataStatistics")
    public Result<DataStatisticsDTO<FinBookRecordDTO>> getMyDataStatistics() {
        log.info("获取获取统计数据[/monthRecord/getMyDataStatistics]请求");
        return Result.success(finBookService.getMyDataStatistics());
    }

    /**
     * 获取指定账户的详细统计数据
     *
     * @param id 账簿ID
     * @return 统计数据DTO
     */
    @PostMapping("/getBookDetailedStatistics")
    public Result<DataStatisticsDTO<FinMonthRecord>> getBookDetailedStatistics(@RequestBody IdDto id) {
        log.info("获取获取统计数据[/monthRecord/getBookDetailedStatistics]请求传参：{}", id);
        return Result.success(finBookService.getBookDetailedStatistics(id));
    }
}
