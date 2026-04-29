package com.dcz.mrecord.service;

import com.dcz.mrecord.dto.DataStatisticsDTO;
import com.dcz.mrecord.dto.IdDto;
import com.dcz.mrecord.dto.QueryFinBookDTO;
import com.dcz.mrecord.entity.FinBook;
import com.mybatisflex.core.paginate.Page;
import com.mybatisflex.core.service.IService;

/**
 * 财务账簿服务
 *
 * @author dcz
 * @since 2026/04/09
 */
public interface FinBookService extends IService<FinBook> {
    /**
     * 创建账簿
     *
     * @param finBook 账簿
     * @return 账簿
     */
    FinBook createFinBook(FinBook finBook);

    /**
     * 更新账簿
     *
     * @param finBook 账簿
     * @return 账簿
     */
    FinBook updateFinBook(FinBook finBook);

    /**
     * 删除账簿
     *
     * @param id 账簿ID
     * @return 账簿
     */
    void deleteFinBook(IdDto id);

    /**
     * 获取我的账簿
     *
     * @param param 查询参数
     * @return 账簿列表
     */
    Page<FinBook> getMyFinBook(QueryFinBookDTO param);


    /**
     * 获取统计数据
     *
     * @return 统计数据DTO
     */
    DataStatisticsDTO getMyDataStatistics();
}
