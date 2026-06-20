//! 响应码枚举
//!
//! 对应 Java: `com.dcz.mrecord.common.ResCode`

/// 预定义业务响应码
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResCode {
    /// 请求成功
    Success,
    /// 请求参数错误
    ParamError,
    /// 用户邮箱已存在，注册失败
    Unauthorized,
    /// 账号或密码错误
    LoginInfoError,
    /// 登录过期
    LoginExpire,
    /// 无操作权限
    NoPermission,
    /// 用户状态异常
    UserStatusError,
    /// 用户账户未激活
    UserNotActivated,
    /// 数据不存在
    DataNotExist,
    /// 数据重复，操作失败
    DataDuplication,
    /// 异步任务处理中
    AsyncProcess,
    /// 操作执行失败
    OperationFail,
    /// 账簿不存在
    FinBookNotFound,
    /// 账簿类型禁止修改
    FinBookTypeUpdate,
    /// 账簿年份禁止修改
    FinBookYearUpdate,
    /// 账目不存在
    FinItemNotFound,
    /// 账目模板不存在
    FinItemTempNotExist,
    /// 账目模板项名称不能为空
    FinItemTempNameRequired,
    /// 账目模板项类型不能为空
    FinItemTempTypeRequired,
    /// 账目模板项类型错误
    FinItemTempTypeError,
    /// 账目模板项仅支持修改名称、图标、顺序
    FinItemTempUpdateError,
    /// 服务器内部错误
    Error,
    /// 业务异常
    BusinessError,
}

impl ResCode {
    /// 响应码字符串（如 "00000"、"10001"）
    pub fn code(self) -> &'static str {
        match self {
            Self::Success => "00000",
            Self::ParamError => "10001",
            Self::Unauthorized => "11001",
            Self::LoginInfoError => "11002",
            Self::LoginExpire => "11003",
            Self::NoPermission => "11004",
            Self::UserStatusError => "11005",
            Self::UserNotActivated => "11006",
            Self::DataNotExist => "12001",
            Self::DataDuplication => "12002",
            Self::AsyncProcess => "13008",
            Self::OperationFail => "13009",
            Self::FinBookNotFound => "14101",
            Self::FinBookTypeUpdate => "14102",
            Self::FinBookYearUpdate => "14103",
            Self::FinItemNotFound => "14201",
            Self::FinItemTempNotExist => "14301",
            Self::FinItemTempNameRequired => "14302",
            Self::FinItemTempTypeRequired => "14303",
            Self::FinItemTempTypeError => "14304",
            Self::FinItemTempUpdateError => "14305",
            Self::Error => "50000",
            Self::BusinessError => "50001",
        }
    }

    /// 响应消息（中文提示）
    pub fn message(self) -> &'static str {
        match self {
            Self::Success => "请求成功",
            Self::ParamError => "请求参数错误",
            Self::Unauthorized => "用户邮箱已存在，注册失败",
            Self::LoginInfoError => "账号或密码错误",
            Self::LoginExpire => "登录过期",
            Self::NoPermission => "无操作权限",
            Self::UserStatusError => "用户状态异常",
            Self::UserNotActivated => "用户账户未激活，请先激活账户",
            Self::DataNotExist => "数据不存在",
            Self::DataDuplication => "数据重复，操作失败",
            Self::AsyncProcess => "异步任务处理中",
            Self::OperationFail => "操作执行失败",
            Self::FinBookNotFound => "账簿不存在",
            Self::FinBookTypeUpdate => "账簿类型禁止修改，如需修改请删除后重新创建，或创建新账簿",
            Self::FinBookYearUpdate => "账簿年份禁止修改，如需修改请删除后重新创建，或创建新账簿",
            Self::FinItemNotFound => "账目不存在",
            Self::FinItemTempNotExist => "账目模板不存在",
            Self::FinItemTempNameRequired => "账目模板项名称不能为空",
            Self::FinItemTempTypeRequired => "账目模板项类型不能为空",
            Self::FinItemTempTypeError => "账目模板项类型错误",
            Self::FinItemTempUpdateError => {
                "账目模板项仅支持修改名称、图标、顺序，无法进行修改类型、删除模板项操作"
            }
            Self::Error => "服务器内部错误",
            Self::BusinessError => "业务异常",
        }
    }
}
