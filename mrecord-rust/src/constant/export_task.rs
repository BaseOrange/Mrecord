//! 导出任务状态常量
//!
//! 对应 Java: `com.dcz.mrecord.constant.ExportTaskStatusConst`

/// 导出任务状态
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExportTaskStatus {
    /// 等待执行
    Wait,
    /// 执行中
    Run,
    /// 成功
    Success,
    /// 失败
    Fail,
}

impl ExportTaskStatus {
    /// 转为数据库存储的字符串
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Wait => "WAIT",
            Self::Run => "RUN",
            Self::Success => "SUCCESS",
            Self::Fail => "FAIL",
        }
    }

    /// 从数据库字符串还原
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "WAIT" => Some(Self::Wait),
            "RUN" => Some(Self::Run),
            "SUCCESS" => Some(Self::Success),
            "FAIL" => Some(Self::Fail),
            _ => None,
        }
    }
}

impl std::fmt::Display for ExportTaskStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
