-- mrecord.fin_book definition

CREATE TABLE `fin_book` (
  `MR_ID` varchar(32) NOT NULL COMMENT '账簿ID，32位UUID字符串',
  `MR_USER_ID` varchar(32) NOT NULL COMMENT '所属用户ID，关联SYS_USER.MR_ID',
  `MR_BOOK_NAME` varchar(64) NOT NULL COMMENT '账簿名称，用户自定义',
  `MR_BOOK_TYPE` varchar(16) NOT NULL COMMENT '账簿类型：YEARLY-年度账簿，CATEGORY-分类账簿',
  `MR_YEAR` int DEFAULT NULL COMMENT '年度账簿对应年份，分类账簿为空',
  `MR_COPY_FROM_BOOK_ID` varchar(32) DEFAULT NULL COMMENT '复制模板源账簿ID，新建年度账簿时关联上一年',
  `MR_CREATE_BY` varchar(32) NOT NULL COMMENT '创建人ID，用户MR_ID',
  `MR_CREATE_TIME` datetime DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
  `MR_UPDATE_BY` varchar(32) NOT NULL COMMENT '更新人ID，用户MR_ID',
  `MR_UPDATE_TIME` datetime DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
  `MR_IS_DELETED` tinyint DEFAULT '0' COMMENT '逻辑删除标识：0-正常，1-已删除',
  PRIMARY KEY (`MR_ID`),
  KEY `IDX_USER_ID` (`MR_USER_ID`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci COMMENT='财务账簿表';


-- mrecord.fin_month_item_record definition

CREATE TABLE `fin_month_item_record` (
  `MR_ID` varchar(32) NOT NULL COMMENT '明细ID，32位UUID字符串',
  `MR_YEAR` int NOT NULL COMMENT '统计年份',
  `MR_MONTH` int NOT NULL COMMENT '统计月份',
  `MR_RECORD_ID` varchar(32) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL COMMENT '关联账簿 ID，FIN_BOOK.MR_ID',
  `MR_TEMPLATE_ITEM_ID` varchar(32) NOT NULL COMMENT '关联模板项ID，FIN_TEMPLATE_ITEM.MR_ID',
  `MR_ITEM_VALUE` decimal(18,2) DEFAULT '0.00' COMMENT '当月该记账项实际金额',
  `MR_CREATE_BY` varchar(32) NOT NULL COMMENT '创建人ID，用户MR_ID',
  `MR_CREATE_TIME` datetime DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
  `MR_UPDATE_BY` varchar(32) NOT NULL COMMENT '更新人ID，用户MR_ID',
  `MR_UPDATE_TIME` datetime DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
  `MR_IS_DELETED` tinyint DEFAULT '0' COMMENT '逻辑删除标识：0-正常，1-已删除',
  PRIMARY KEY (`MR_ID`),
  UNIQUE KEY `UK_RECORD_ITEM` (`MR_YEAR`,`MR_MONTH`,`MR_RECORD_ID`,`MR_TEMPLATE_ITEM_ID`),
  KEY `IDX_RECORD_ID` (`MR_RECORD_ID`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci COMMENT='月度财务明细项表';


-- mrecord.fin_month_record definition

CREATE TABLE `fin_month_record` (
  `MR_ID` varchar(32) NOT NULL COMMENT '月度汇总ID，32位UUID字符串',
  `MR_USER_ID` varchar(32) NOT NULL COMMENT '所属用户ID，关联SYS_USER.MR_ID',
  `MR_BOOK_ID` varchar(32) NOT NULL COMMENT '所属账簿ID，关联FIN_BOOK.MR_ID',
  `MR_YEAR` int NOT NULL COMMENT '统计年份',
  `MR_MONTH` int NOT NULL COMMENT '统计月份',
  `MR_TOTAL_ASSET` decimal(18,2) DEFAULT '0.00' COMMENT '当月总资产，所有资产项之和',
  `MR_TOTAL_LIABILITY` decimal(18,2) DEFAULT '0.00' COMMENT '当月总负债，所有负债项之和',
  `MR_NET_ASSET` decimal(18,2) DEFAULT '0.00' COMMENT '当月净资产，总资产-总负债',
  `MR_MONTH_ON_MONTH` decimal(18,2) DEFAULT NULL COMMENT '环比增长/下跌金额，对比上月',
  `MR_YEAR_ON_YEAR` decimal(18,2) DEFAULT NULL COMMENT '同比增长/下跌金额，对比去年同月',
  `MR_NOTE` varchar(500) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci DEFAULT NULL COMMENT '用户本月汇总备注',
  `MR_CREATE_BY` varchar(32) NOT NULL COMMENT '创建人ID，用户MR_ID',
  `MR_CREATE_TIME` datetime DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
  `MR_UPDATE_BY` varchar(32) NOT NULL COMMENT '更新人ID，用户MR_ID',
  `MR_UPDATE_TIME` datetime DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
  `MR_IS_DELETED` tinyint DEFAULT '0' COMMENT '逻辑删除标识：0-正常，1-已删除',
  PRIMARY KEY (`MR_ID`),
  UNIQUE KEY `UK_BOOK_YEAR_MONTH` (`MR_BOOK_ID`,`MR_YEAR`,`MR_MONTH`),
  KEY `IDX_USER_ID` (`MR_USER_ID`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci COMMENT='月度财务汇总表';


-- mrecord.fin_template_item definition

CREATE TABLE `fin_template_item` (
  `MR_ID` varchar(32) NOT NULL COMMENT '模板项ID，32位UUID字符串',
  `MR_BOOK_ID` varchar(32) NOT NULL COMMENT '所属账簿ID，关联FIN_BOOK.MR_ID',
  `MR_ITEM_NAME` varchar(64) NOT NULL COMMENT '记账项名称，如招行储蓄卡、花呗',
  `MR_ITEM_TYPE` varchar(16) NOT NULL COMMENT '记账项类型：ASSET-资产，LIABILITY-负债',
  `MR_ICON` varchar(64) DEFAULT NULL COMMENT '图标标识，对应系统内置图标库',
  `MR_SORT` int DEFAULT '0' COMMENT '展示排序号，数值越小越靠前',
  `MR_CREATE_BY` varchar(32) NOT NULL COMMENT '创建人ID，用户MR_ID',
  `MR_CREATE_TIME` datetime DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
  `MR_UPDATE_BY` varchar(32) NOT NULL COMMENT '更新人ID，用户MR_ID',
  `MR_UPDATE_TIME` datetime DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
  `MR_IS_DELETED` tinyint DEFAULT '0' COMMENT '逻辑删除标识：0-正常，1-已删除',
  PRIMARY KEY (`MR_ID`),
  KEY `IDX_BOOK_ID` (`MR_BOOK_ID`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci COMMENT='记账模板明细表';


-- mrecord.sys_backup_book definition

CREATE TABLE `sys_backup_book` (
  `MR_ID` varchar(32) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL COMMENT '账簿ID，32位UUID字符串',
  `MR_USER_ID` varchar(32) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL COMMENT '所属用户ID，关联SYS_USER.MR_ID',
  `MR_BOOK_NAME` varchar(64) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL COMMENT '账簿名称，用户自定义',
  `MR_BOOK_TYPE` varchar(16) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL COMMENT '账簿类型：YEARLY-年度账簿，CATEGORY-分类账簿',
  `MR_YEAR` int DEFAULT NULL COMMENT '年度账簿对应年份，分类账簿为空',
  `MR_COPY_FROM_BOOK_ID` varchar(32) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci DEFAULT NULL COMMENT '复制模板源账簿ID，新建年度账簿时关联上一年',
  `MR_CREATE_BY` varchar(32) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL COMMENT '创建人ID，用户MR_ID',
  `MR_CREATE_TIME` datetime DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
  `MR_UPDATE_BY` varchar(32) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL COMMENT '更新人ID，用户MR_ID',
  `MR_UPDATE_TIME` datetime DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
  `MR_IS_DELETED` tinyint DEFAULT '0' COMMENT '逻辑删除标识：0-正常，1-已删除',
  PRIMARY KEY (`MR_ID`),
  KEY `IDX_USER_ID` (`MR_USER_ID`) USING BTREE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci COMMENT='财务账簿备份表';


-- mrecord.sys_backup_month_item_record definition

CREATE TABLE `sys_backup_month_item_record` (
  `MR_ID` varchar(32) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL COMMENT '明细ID，32位UUID字符串',
  `MR_YEAR` int NOT NULL COMMENT '统计年份',
  `MR_MONTH` int NOT NULL COMMENT '统计月份',
  `MR_BOOK_ID` varchar(32) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL COMMENT '关联账簿 ID，FIN_BOOK.MR_ID',
  `MR_TEMPLATE_ITEM_ID` varchar(32) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL COMMENT '关联模板项ID，FIN_TEMPLATE_ITEM.MR_ID',
  `MR_ITEM_VALUE` decimal(18,2) DEFAULT '0.00' COMMENT '当月该记账项实际金额',
  `MR_CREATE_BY` varchar(32) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL COMMENT '创建人ID，用户MR_ID',
  `MR_CREATE_TIME` datetime DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
  `MR_UPDATE_BY` varchar(32) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL COMMENT '更新人ID，用户MR_ID',
  `MR_UPDATE_TIME` datetime DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
  `MR_IS_DELETED` tinyint DEFAULT '0' COMMENT '逻辑删除标识：0-正常，1-已删除',
  PRIMARY KEY (`MR_ID`),
  UNIQUE KEY `UK_RECORD_ITEM` (`MR_YEAR`,`MR_MONTH`,`MR_BOOK_ID`,`MR_TEMPLATE_ITEM_ID`),
  KEY `IDX_RECORD_ID` (`MR_BOOK_ID`) USING BTREE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci COMMENT='月度财务明细项备份表';


-- mrecord.sys_backup_month_record definition

CREATE TABLE `sys_backup_month_record` (
  `MR_ID` varchar(32) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL COMMENT '月度汇总ID，32位UUID字符串',
  `MR_USER_ID` varchar(32) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL COMMENT '所属用户ID，关联SYS_USER.MR_ID',
  `MR_BOOK_ID` varchar(32) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL COMMENT '所属账簿ID，关联FIN_BOOK.MR_ID',
  `MR_YEAR` int NOT NULL COMMENT '统计年份',
  `MR_MONTH` int NOT NULL COMMENT '统计月份',
  `MR_TOTAL_ASSET` decimal(18,2) DEFAULT '0.00' COMMENT '当月总资产，所有资产项之和',
  `MR_TOTAL_LIABILITY` decimal(18,2) DEFAULT '0.00' COMMENT '当月总负债，所有负债项之和',
  `MR_NET_ASSET` decimal(18,2) DEFAULT '0.00' COMMENT '当月净资产，总资产-总负债',
  `MR_MONTH_ON_MONTH` decimal(18,2) DEFAULT NULL COMMENT '环比增长/下跌金额，对比上月',
  `MR_YEAR_ON_YEAR` decimal(18,2) DEFAULT NULL COMMENT '同比增长/下跌金额，对比去年同月',
  `MR_NOTE` varchar(500) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci DEFAULT NULL COMMENT '用户本月汇总备注',
  `MR_CREATE_BY` varchar(32) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL COMMENT '创建人ID，用户MR_ID',
  `MR_CREATE_TIME` datetime DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
  `MR_UPDATE_BY` varchar(32) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL COMMENT '更新人ID，用户MR_ID',
  `MR_UPDATE_TIME` datetime DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
  `MR_IS_DELETED` tinyint DEFAULT '0' COMMENT '逻辑删除标识：0-正常，1-已删除',
  PRIMARY KEY (`MR_ID`),
  UNIQUE KEY `UK_BOOK_YEAR_MONTH` (`MR_BOOK_ID`,`MR_YEAR`,`MR_MONTH`),
  KEY `IDX_USER_ID` (`MR_USER_ID`) USING BTREE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci COMMENT='月度财务汇总备份表';


-- mrecord.sys_backup_template_item definition

CREATE TABLE `sys_backup_template_item` (
  `MR_ID` varchar(32) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL COMMENT '模板项ID，32位UUID字符串',
  `MR_BOOK_ID` varchar(32) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL COMMENT '所属账簿ID，关联FIN_BOOK.MR_ID',
  `MR_ITEM_NAME` varchar(64) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL COMMENT '记账项名称，如招行储蓄卡、花呗',
  `MR_ITEM_TYPE` varchar(16) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL COMMENT '记账项类型：ASSET-资产，LIABILITY-负债',
  `MR_ICON` varchar(64) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci DEFAULT NULL COMMENT '图标标识，对应系统内置图标库',
  `MR_SORT` int DEFAULT '0' COMMENT '展示排序号，数值越小越靠前',
  `MR_CREATE_BY` varchar(32) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL COMMENT '创建人ID，用户MR_ID',
  `MR_CREATE_TIME` datetime DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
  `MR_UPDATE_BY` varchar(32) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL COMMENT '更新人ID，用户MR_ID',
  `MR_UPDATE_TIME` datetime DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
  `MR_IS_DELETED` tinyint DEFAULT '0' COMMENT '逻辑删除标识：0-正常，1-已删除',
  PRIMARY KEY (`MR_ID`),
  KEY `IDX_BOOK_ID` (`MR_BOOK_ID`) USING BTREE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci COMMENT='记账模板明细备份表';


-- mrecord.sys_backup_user definition

CREATE TABLE `sys_backup_user` (
  `MR_ID` varchar(32) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL COMMENT '用户ID，32位UUID字符串',
  `MR_EMAIL` varchar(64) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL COMMENT '登录邮箱，唯一不重复',
  `MR_PASSWORD` varchar(128) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL COMMENT '用户登录密码，BCrypt加密存储',
  `MR_NICKNAME` varchar(32) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci DEFAULT NULL COMMENT '用户自定义昵称',
  `MR_STATUS` tinyint NOT NULL DEFAULT '0' COMMENT '账号状态：0-正常，1-停用，2-注销待生效，3-已注销',
  `MR_CANCEL_TIME` datetime DEFAULT NULL COMMENT '账号注销申请时间，用于计算15天冷静期',
  `MR_REMIND_ENABLED` tinyint DEFAULT '0' COMMENT '月度邮件提醒开关：0-关闭，1-开启',
  `MR_REMIND_DAY` int DEFAULT '1' COMMENT '月度提醒日期，取值1-31，无对应日期取月末',
  `MR_CREATE_BY` varchar(32) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL COMMENT '创建人ID，对应用户MR_ID',
  `MR_CREATE_TIME` datetime DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
  `MR_UPDATE_BY` varchar(32) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL COMMENT '更新人ID，对应用户MR_ID',
  `MR_UPDATE_TIME` datetime DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
  `MR_IS_DELETED` tinyint DEFAULT '0' COMMENT '逻辑删除标识：0-正常，1-已删除'
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci COMMENT='用户信息表';


-- mrecord.sys_config definition

CREATE TABLE `sys_config` (
  `MR_ID` varchar(32) NOT NULL COMMENT '配置ID，32位UUID字符串',
  `MR_CONFIG_KEY` varchar(64) NOT NULL COMMENT '配置键名，代码读取唯一标识',
  `MR_CONFIG_VALUE` varchar(512) DEFAULT NULL COMMENT '配置参数值',
  `MR_REMARK` varchar(128) DEFAULT NULL COMMENT '配置项说明，解释配置用途',
  `MR_CREATE_BY` varchar(32) NOT NULL COMMENT '创建人ID，管理员用户MR_ID',
  `MR_CREATE_TIME` datetime DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
  `MR_UPDATE_BY` varchar(32) NOT NULL COMMENT '更新人ID，管理员用户MR_ID',
  `MR_UPDATE_TIME` datetime DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
  PRIMARY KEY (`MR_ID`),
  UNIQUE KEY `UK_CONFIG_KEY` (`MR_CONFIG_KEY`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci COMMENT='系统全局配置表';


-- mrecord.sys_export_task definition

CREATE TABLE `sys_export_task` (
  `MR_ID` varchar(32) NOT NULL COMMENT '任务ID，32位UUID字符串',
  `MR_USER_ID` varchar(32) NOT NULL COMMENT '发起用户ID，关联SYS_USER.MR_ID',
  `MR_BOOK_ID` varchar(32) NOT NULL COMMENT '导出账簿ID，关联FIN_BOOK.MR_ID',
  `MR_BOOK_TYPE` varchar(16) NOT NULL COMMENT '账簿类型：YEARLY-年度，CATEGORY-分类',
  `MR_START_YEAR_MONTH` varchar(8) NOT NULL COMMENT '导出开始年月，格式yyyyMM',
  `MR_END_YEAR_MONTH` varchar(8) NOT NULL COMMENT '导出结束年月，格式yyyyMM',
  `MR_STATUS` varchar(16) DEFAULT 'WAIT' COMMENT '任务状态：WAIT-待执行，RUN-执行中，SUCCESS-成功，FAIL-失败',
  `MR_FILE_NAME` varchar(128) DEFAULT NULL COMMENT '生成的Excel文件名',
  `MR_FAIL_REASON` varchar(255) DEFAULT NULL COMMENT '任务失败原因，失败时填充',
  `MR_CREATE_BY` varchar(32) NOT NULL COMMENT '创建人ID，发起用户MR_ID',
  `MR_CREATE_TIME` datetime DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
  `MR_UPDATE_BY` varchar(32) NOT NULL COMMENT '更新人ID，系统/用户',
  `MR_UPDATE_TIME` datetime DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
  `MR_IS_DELETED` tinyint DEFAULT '0' COMMENT '逻辑删除标识：0-正常，1-已删除',
  PRIMARY KEY (`MR_ID`),
  KEY `IDX_USER_ID` (`MR_USER_ID`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci COMMENT='异步导出任务表';


-- mrecord.sys_user definition

CREATE TABLE `sys_user` (
  `MR_ID` varchar(32) NOT NULL COMMENT '用户ID，32位UUID字符串',
  `MR_EMAIL` varchar(64) NOT NULL COMMENT '登录邮箱，唯一不重复',
  `MR_PASSWORD` varchar(128) NOT NULL COMMENT '用户登录密码，BCrypt加密存储',
  `MR_NICKNAME` varchar(32) DEFAULT NULL COMMENT '用户自定义昵称',
  `MR_ADMIN` tinyint NOT NULL DEFAULT '0' COMMENT '是否为管理员：0-普通用户，1-管理员',
  `MR_STATUS` tinyint NOT NULL DEFAULT '0' COMMENT '账号状态：0-正常，1-停用，2-注销待生效，3-已注销',
  `MR_CANCEL_TIME` datetime DEFAULT NULL COMMENT '账号注销申请时间，用于计算15天冷静期',
  `MR_REMIND_ENABLED` tinyint DEFAULT '0' COMMENT '月度邮件提醒开关：0-关闭，1-开启',
  `MR_REMIND_DAY` int DEFAULT '1' COMMENT '月度提醒日期，取值1-31，无对应日期取月末',
  `MR_CREATE_BY` varchar(32) NOT NULL COMMENT '创建人ID，对应用户MR_ID',
  `MR_CREATE_TIME` datetime DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
  `MR_UPDATE_BY` varchar(32) NOT NULL COMMENT '更新人ID，对应用户MR_ID',
  `MR_UPDATE_TIME` datetime DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
  `MR_IS_DELETED` tinyint DEFAULT '0' COMMENT '逻辑删除标识：0-正常，1-已删除',
  PRIMARY KEY (`MR_ID`),
  UNIQUE KEY `UK_EMAIL` (`MR_EMAIL`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci COMMENT='用户信息表';


-- mrecord.sys_user_operate_log definition

CREATE TABLE `sys_user_operate_log` (
  `MR_ID` varchar(32) NOT NULL COMMENT '日志ID，32位UUID字符串',
  `MR_USER_ID` varchar(32) NOT NULL COMMENT '操作用户ID，关联SYS_USER.MR_ID',
  `MR_OPERATE_TYPE` varchar(32) NOT NULL COMMENT '操作类型：LOGIN-登录，LOGOUT-登出，UPDATE-数据修改，EXPORT-导出，CANCEL-注销/撤销注销，RESET_PWD-密码重置',
  `MR_CONTENT` varchar(255) DEFAULT NULL COMMENT '操作内容详细描述',
  `MR_IP` varchar(32) DEFAULT NULL COMMENT '操作IP地址',
  `MR_CREATE_BY` varchar(32) NOT NULL COMMENT '创建人ID，即操作用户MR_ID',
  `MR_CREATE_TIME` datetime DEFAULT CURRENT_TIMESTAMP COMMENT '操作时间',
  `MR_UPDATE_BY` varchar(32) DEFAULT NULL COMMENT '更新人ID，日志一般不更新',
  `MR_UPDATE_TIME` datetime DEFAULT NULL COMMENT '更新时间',
  `MR_IS_DELETED` tinyint DEFAULT '0' COMMENT '逻辑删除标识：0-正常，1-已删除',
  PRIMARY KEY (`MR_ID`),
  KEY `IDX_USER_ID` (`MR_USER_ID`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci COMMENT='用户操作审计日志表';