import { post } from '@/utils/request';
// ==================== 接口方法 ====================
/** 发起账簿数据导出任务 */
export function exportBookData(data) {
    return post('/exportTask/export', data);
}
/** 查询当前用户的导出任务列表 */
export function listExportTasks(data) {
    return post('/exportTask/list', data);
}
