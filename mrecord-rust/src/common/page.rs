//! 分页响应封装
//!
//! 对应 Java: `com.mybatisflex.core.paginate.Page`

use serde::Serialize;

/// 通用分页结果
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PageResult<T: Serialize> {
    /// 当前页数据
    pub records: Vec<T>,
    /// 总记录数
    pub total_row: u64,
    /// 当前页码
    pub page_number: u64,
    /// 每页数量
    pub page_size: u64,
    /// 总页数
    pub total_page: u64,
}

impl<T: Serialize> PageResult<T> {
    /// 构造分页结果
    pub fn new(records: Vec<T>, total_row: u64, page_number: u64, page_size: u64) -> Self {
        // 总页数：向上取整；page_size 为 0 时降级为 0，避免除零
        let total_page = if page_size == 0 {
            0
        } else {
            total_row.div_ceil(page_size)
        };
        Self {
            records,
            total_row,
            page_number,
            page_size,
            total_page,
        }
    }
}
