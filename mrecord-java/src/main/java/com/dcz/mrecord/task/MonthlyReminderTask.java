package com.dcz.mrecord.task;

import com.dcz.mrecord.bo.MailParamsBO;
import com.dcz.mrecord.constant.UserStatusConst;
import com.dcz.mrecord.entity.SysUser;
import com.dcz.mrecord.mapper.SysUserMapper;
import com.dcz.mrecord.service.EmailService;
import com.mybatisflex.core.query.QueryWrapper;
import jakarta.annotation.Resource;
import lombok.extern.slf4j.Slf4j;
import org.springframework.scheduling.annotation.Scheduled;
import org.springframework.stereotype.Component;

import java.time.LocalDate;
import java.time.temporal.TemporalAdjusters;
import java.util.ArrayList;
import java.util.HashSet;
import java.util.List;
import java.util.Set;

/**
 * 月度记账提醒定时任务
 * 每天早上 8:08 执行，向设置了本日提醒的用户发送邮件
 *
 * @author dcz
 * @since 2026/05/12
 */
@Slf4j
@Component
public class MonthlyReminderTask {

    @Resource
    private SysUserMapper sysUserMapper;

    @Resource
    private EmailService emailService;

    /**
     * 每月提醒任务
     * cron: 每天早上 8:08 执行
     */
    @Scheduled(cron = "0 8 8 * * ?")
    public void monthlyReminder() {
        log.info("【月度提醒定时任务】开始执行...");

        try {
            LocalDate today = LocalDate.now();
            int dayOfMonth = today.getDayOfMonth();
            int lastDayOfMonth = today.with(TemporalAdjusters.lastDayOfMonth()).getDayOfMonth();
            boolean isLastDay = dayOfMonth == lastDayOfMonth;

            log.info("【月度提醒定时任务】当前日期: {}, 本月最后一天: {}, 是否月末: {}", today, lastDayOfMonth, isLastDay);

            // 查询 remindDay 等于今日的用户
            List<SysUser> users = queryUsersByRemindDay(dayOfMonth);

            // 若今日是月末，额外查询 remindDay 大于今日且在 1-31 范围内的用户
            if (isLastDay) {
                List<SysUser> extraUsers = queryUsersByRemindDayRange(dayOfMonth + 1, 31);
                users = mergeUserList(users, extraUsers);
            }

            if (users.isEmpty()) {
                log.info("【月度提醒定时任务】今日无需要提醒的用户");
                return;
            }

            log.info("【月度提醒定时任务】今日需提醒用户数: {}", users.size());

            // 组装邮件参数并发送
            List<MailParamsBO> paramsList = buildMailParams(users);
            emailService.sendMonthReportEmail(paramsList);

            log.info("【月度提醒定时任务】执行完成，共发送 {} 条提醒", paramsList.size());
        } catch (Exception e) {
            log.error("【月度提醒定时任务】执行异常", e);
        }
    }

    /**
     * 根据提醒日期查询用户
     *
     * @param remindDay 提醒日期
     * @return 用户列表
     */
    private List<SysUser> queryUsersByRemindDay(int remindDay) {
        QueryWrapper qw = QueryWrapper.create()
                .and(SysUser::getIsDeleted).eq(0)
                .and(SysUser::getStatus).eq(UserStatusConst.NORMAL)
                .and(SysUser::getRemindEnabled).eq(1)
                .and(SysUser::getRemindDay).eq(remindDay);
        return sysUserMapper.selectListByQuery(qw);
    }

    /**
     * 根据提醒日期范围查询用户（用于月末兜底）
     *
     * @param startDay 开始日期（含）
     * @param endDay   结束日期（含）
     * @return 用户列表
     */
    private List<SysUser> queryUsersByRemindDayRange(int startDay, int endDay) {
        QueryWrapper qw = QueryWrapper.create()
                .and(SysUser::getIsDeleted).eq(0)
                .and(SysUser::getStatus).eq(UserStatusConst.NORMAL)
                .and(SysUser::getRemindEnabled).eq(1)
                .and(SysUser::getRemindDay).ge(startDay)
                .and(SysUser::getRemindDay).le(endDay);
        return sysUserMapper.selectListByQuery(qw);
    }

    /**
     * 合并用户列表并去重
     *
     * @param list1 列表1
     * @param list2 列表2
     * @return 去重后的用户列表
     */
    private List<SysUser> mergeUserList(List<SysUser> list1, List<SysUser> list2) {
        Set<String> idSet = new HashSet<>();
        List<SysUser> result = new ArrayList<>();

        for (SysUser user : list1) {
            if (idSet.add(user.getId())) {
                result.add(user);
            }
        }
        for (SysUser user : list2) {
            if (idSet.add(user.getId())) {
                result.add(user);
            }
        }
        return result;
    }

    /**
     * 构建邮件参数列表
     *
     * @param users 用户列表
     * @return 邮件参数列表
     */
    private List<MailParamsBO> buildMailParams(List<SysUser> users) {
        List<MailParamsBO> paramsList = new ArrayList<>();
        for (SysUser user : users) {
            MailParamsBO params = new MailParamsBO();
            params.setTo(user.getEmail());
            params.setUserName(user.getNickname());
            params.setUserEmail(user.getEmail());
            paramsList.add(params);
        }
        return paramsList;
    }
}
