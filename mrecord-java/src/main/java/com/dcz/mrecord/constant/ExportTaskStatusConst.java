package com.dcz.mrecord.constant;

/**
 * 导出任务状态常量
 *
 * @author dcz
 */
public final class ExportTaskStatusConst {

    private ExportTaskStatusConst() {
    }

    /** 等待执行 */
    public static final String WAIT = "WAIT";

    /** 执行中 */
    public static final String RUN = "RUN";

    /** 成功 */
    public static final String SUCCESS = "SUCCESS";

    /** 失败 */
    public static final String FAIL = "FAIL";
}
