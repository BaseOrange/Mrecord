package com.dcz.mrecord.task;

import cn.hutool.core.date.DateUtil;
import com.dcz.mrecord.constant.UserStatusConst;
import com.dcz.mrecord.entity.SysUser;
import com.dcz.mrecord.mapper.SysUserMapper;
import com.dcz.mrecord.service.SysUserService;
import com.mybatisflex.core.query.QueryWrapper;
import jakarta.annotation.Resource;
import lombok.extern.slf4j.Slf4j;
import org.springframework.scheduling.annotation.Scheduled;
import org.springframework.stereotype.Component;

import java.util.Date;
import java.util.List;

/**
 * 用户注销清理定时任务
 * 每天凌晨 3:00 执行，扫描「注销待生效」且已超过冷静期的用户，清理其全部数据后物理删除用户
 *
 * <p>{@code SysUserServiceImpl.canceledMyUser} 只负责把用户置为「注销待生效」并记录 cancelTime，
 * 真正的清理由本任务完成。对应 Rust 端 {@code service::cancel_cleanup_task::CancelCleanupTask}。</p>
 *
 * @author dcz
 * @since 2026/09/18
 */
@Slf4j
@Component
public class CancelCleanupTask {

    /**
     * 注销冷静期（天）：与 Rust 端 {@code COOLING_PERIOD_DAYS} 保持一致
     */
    private static final int COOLING_PERIOD_DAYS = 15;

    @Resource
    private SysUserMapper sysUserMapper;

    @Resource
    private SysUserService sysUserService;

    /**
     * 注销清理任务
     * cron: 每天凌晨 3:00 执行（避开 8:08 的月度提醒任务）
     */
    @Scheduled(cron = "0 0 3 * * ?")
    public void cancelCleanup() {
        log.info("【注销清理定时任务】开始执行...");

        try {
            Date deadline = DateUtil.offsetDay(new Date(), -COOLING_PERIOD_DAYS);
            log.info("【注销清理定时任务】冷静期截止时间: {}", deadline);

            // 查询待注销且已过冷静期的用户（cancelTime 为空的不参与比较，不会被清理）
            QueryWrapper qw = QueryWrapper.create()
                    .and(SysUser::getIsDeleted).eq(0)
                    .and(SysUser::getStatus).eq(UserStatusConst.CANCELED_WAIT)
                    .and(SysUser::getCancelTime).le(deadline);
            List<SysUser> users = sysUserMapper.selectListByQuery(qw);

            if (users == null || users.isEmpty()) {
                log.info("【注销清理定时任务】无可清理用户");
                return;
            }

            log.info("【注销清理定时任务】匹配到 {} 位待注销用户", users.size());

            int cleaned = 0;
            for (SysUser user : users) {
                try {
                    sysUserService.cleanupCanceledUser(user);
                    cleaned++;
                    log.info("【注销清理定时任务】用户 {} 注销清理完成", user.getId());
                } catch (Exception e) {
                    // 单用户失败不影响其它用户，整体继续
                    log.error("【注销清理定时任务】用户 {} 注销清理失败，已跳过", user.getId(), e);
                }
            }

            log.info("【注销清理定时任务】执行完成，共清理 {} 位用户", cleaned);
        } catch (Exception e) {
            log.error("【注销清理定时任务】执行异常", e);
        }
    }
}
