package com.dcz.mrecord.task;

import com.dcz.mrecord.bo.MailParamsBO;
import com.dcz.mrecord.constant.UserStatusConst;
import com.dcz.mrecord.entity.FinBook;
import com.dcz.mrecord.entity.FinMonthItemRecord;
import com.dcz.mrecord.entity.FinMonthRecord;
import com.dcz.mrecord.entity.SysUser;
import com.dcz.mrecord.mapper.FinBookMapper;
import com.dcz.mrecord.mapper.FinMonthItemRecordMapper;
import com.dcz.mrecord.mapper.FinMonthRecordMapper;
import com.dcz.mrecord.mapper.SysUserMapper;
import com.dcz.mrecord.service.EmailService;
import com.mybatisflex.core.query.QueryWrapper;
import jakarta.annotation.Resource;
import lombok.extern.slf4j.Slf4j;
import org.springframework.scheduling.annotation.Scheduled;
import org.springframework.stereotype.Component;

import java.math.BigDecimal;
import java.math.RoundingMode;
import java.text.DecimalFormat;
import java.text.DecimalFormatSymbols;
import java.time.LocalDate;
import java.util.ArrayList;
import java.util.LinkedHashMap;
import java.util.List;
import java.util.Locale;
import java.util.Map;
import java.util.Optional;
import java.util.TreeMap;
import java.util.stream.Collectors;

/**
 * 年度总结邮件定时任务
 * 每年 1 月 1 日 08:08 执行，为用户发送上一年的年度财务总结邮件
 *
 * <p>原 {@code EmailService.sendNewYearReminderEmail}（新财年提醒）无任何调用入口，
 * 产品决策不做新财年功能，改为年度总结。本任务为 Java / Rust 两侧共同补齐的业务定义，
 * 对应 Rust 端 {@code service::yearly_summary_task::YearlySummaryTask}。</p>
 *
 * <p>执行时机为每年元旦 08:08，此时 {@code LocalDate.now()} 已是新年份，
 * 因此总结年份 = 当前年份 - 1（如 2026-01-01 08:08 总结 2025 年全年数据）。</p>
 *
 * <p>收件人与月度提醒任务保持一致：状态正常、未删除且开启邮件提醒的用户。
 * 用户在册账簿为空或该年度无月度汇总记录时跳过（不发送空洞的「0 个月」总结）。</p>
 *
 * @author dcz
 * @since 2026/09/18
 */
@Slf4j
@Component
public class YearlySummaryTask {

    /**
     * 金额展示格式：两位小数 + 千分位分隔符（HALF_UP）
     */
    private static final DecimalFormat MONEY_FORMAT;

    static {
        DecimalFormatSymbols symbols = DecimalFormatSymbols.getInstance(Locale.CHINA);
        MONEY_FORMAT = new DecimalFormat("#,##0.00", symbols);
    }

    @Resource
    private SysUserMapper sysUserMapper;

    @Resource
    private FinBookMapper finBookMapper;

    @Resource
    private FinMonthRecordMapper finMonthRecordMapper;

    @Resource
    private FinMonthItemRecordMapper finMonthItemRecordMapper;

    @Resource
    private EmailService emailService;

    /**
     * 年度总结任务
     * cron: 每年 1 月 1 日 08:08 执行（秒 分 时 日 月 周）
     */
    @Scheduled(cron = "0 8 8 1 1 ?")
    public void yearlySummary() {
        log.info("【年度总结定时任务】开始执行...");

        try {
            // 每年 1 月 1 日执行，总结年份为刚结束的一年
            int summaryYear = LocalDate.now().getYear() - 1;
            List<SysUser> users = querySummaryUsers();
            if (users == null || users.isEmpty()) {
                log.info("【年度总结定时任务】无可接收年度总结的用户");
                return;
            }

            log.info("【年度总结定时任务】总结年份: {}, 候选用户数: {}", summaryYear, users.size());

            List<MailParamsBO> paramsList = new ArrayList<>();
            for (SysUser user : users) {
                try {
                    Optional<MailParamsBO> params = buildYearSummaryParams(user, summaryYear);
                    params.ifPresent(paramsList::add);
                } catch (Exception e) {
                    // 单用户失败不影响其它用户
                    log.error("【年度总结定时任务】用户 {} 年度总结生成失败，已跳过", user.getId(), e);
                }
            }

            if (!paramsList.isEmpty()) {
                emailService.sendYearSummaryEmail(paramsList);
            }

            log.info("【年度总结定时任务】执行完成，共发送 {} 条年度总结", paramsList.size());
        } catch (Exception e) {
            log.error("【年度总结定时任务】执行异常", e);
        }
    }

    /**
     * 查询可接收年度总结的用户：正常、未删除、已开启邮件提醒
     */
    private List<SysUser> querySummaryUsers() {
        QueryWrapper qw = QueryWrapper.create()
                .and(SysUser::getIsDeleted).eq(0)
                .and(SysUser::getStatus).eq(UserStatusConst.NORMAL)
                .and(SysUser::getRemindEnabled).eq(1);
        return sysUserMapper.selectListByQuery(qw);
    }

    /**
     * 聚合用户在指定年度的财务数据，生成年度总结邮件参数
     *
     * @return 用户无账簿或该年度无月度汇总时返回 empty
     */
    private Optional<MailParamsBO> buildYearSummaryParams(SysUser user, int year) {
        // 在册账簿
        QueryWrapper bookQw = QueryWrapper.create()
                .and(FinBook::getUserId).eq(user.getId())
                .and(FinBook::getIsDeleted).eq(0);
        List<FinBook> books = finBookMapper.selectListByQuery(bookQw);
        if (books == null || books.isEmpty()) {
            return Optional.empty();
        }
        List<String> bookIds = books.stream().map(FinBook::getId).collect(Collectors.toList());

        // 年度月度汇总（按月份、更新时间升序，用于同账簿同月去重时保留最新一条）
        QueryWrapper recordQw = QueryWrapper.create()
                .and(FinMonthRecord::getUserId).eq(user.getId())
                .and(FinMonthRecord::getYear).eq(year)
                .and(FinMonthRecord::getIsDeleted).eq(0)
                .orderBy(FinMonthRecord::getMonth, true)
                .orderBy(FinMonthRecord::getUpdateTime, true);
        List<FinMonthRecord> records = finMonthRecordMapper.selectListByQuery(recordQw);
        if (records == null || records.isEmpty()) {
            return Optional.empty();
        }

        // 同一（账簿, 月份）可能存在历史重复行：按 (月份, 更新时间) 升序遍历，后者覆盖前者
        Map<String, FinMonthRecord> latestByBookMonth = new LinkedHashMap<>();
        for (FinMonthRecord record : records) {
            latestByBookMonth.put(record.getBookId() + "#" + record.getMonth(), record);
        }

        // 跨账簿按月合计：TreeMap 保证按月份升序，首项为「年初」、末项为「年末」
        TreeMap<Integer, BigDecimal[]> monthly = new TreeMap<>();
        for (FinMonthRecord record : latestByBookMonth.values()) {
            BigDecimal[] agg = monthly.computeIfAbsent(record.getMonth(),
                    k -> new BigDecimal[]{BigDecimal.ZERO, BigDecimal.ZERO, BigDecimal.ZERO});
            agg[0] = agg[0].add(nz(record.getTotalAsset()));
            agg[1] = agg[1].add(nz(record.getTotalLiability()));
            agg[2] = agg[2].add(nz(record.getNetAsset()));
        }

        BigDecimal[] startAgg = monthly.firstEntry().getValue();
        BigDecimal[] endAgg = monthly.lastEntry().getValue();

        // 年度明细记录条数
        QueryWrapper itemQw = QueryWrapper.create()
                .and(FinMonthItemRecord::getYear).eq(year)
                .and(FinMonthItemRecord::getBookId).in(bookIds)
                .and(FinMonthItemRecord::getIsDeleted).eq(0);
        long itemCount = finMonthItemRecordMapper.selectCountByQuery(itemQw);

        MailParamsBO params = new MailParamsBO();
        params.setTo(user.getEmail());
        params.setUserName(user.getNickname());
        params.setUserEmail(user.getEmail());
        params.setSummaryYear(String.valueOf(year));
        params.setRecordedMonths(String.valueOf(monthly.size()));
        params.setBookCount(String.valueOf(books.size()));
        params.setItemCount(String.valueOf(itemCount));
        params.setTotalAsset(formatMoney(endAgg[0]));
        params.setTotalLiability(formatMoney(endAgg[1]));
        params.setNetAsset(formatMoney(endAgg[2]));
        params.setNetAssetStart(formatMoney(startAgg[2]));

        // 净资产变化金额：正数带「+」，负数自带「-」
        BigDecimal change = endAgg[2].subtract(startAgg[2]).setScale(2, RoundingMode.HALF_UP);
        params.setNetAssetChange(formatMoneySigned(change));

        // 年初净资产为零时无对比基准，变化率展示为「—」
        if (startAgg[2].compareTo(BigDecimal.ZERO) == 0) {
            params.setNetAssetChangeRate("—");
        } else {
            BigDecimal rate = calcGrowthRate(endAgg[2], startAgg[2]);
            params.setNetAssetChangeRate((rate.signum() >= 0 ? "+" : "") + rate.toPlainString() + "%");
        }

        return Optional.of(params);
    }

    /**
     * 计算增长率：(本期 - 基期) / |基期| * 100，中间除法保留 4 位、最终保留 2 位（HALF_UP）
     *
     * <p>与 {@code FinMonthRecordServiceImpl.getMonthOnMonthVal} 的舍入链路一致。</p>
     */
    private BigDecimal calcGrowthRate(BigDecimal current, BigDecimal base) {
        return current.subtract(base)
                .divide(base.abs(), 4, RoundingMode.HALF_UP)
                .multiply(BigDecimal.valueOf(100))
                .setScale(2, RoundingMode.HALF_UP);
    }

    /**
     * 金额展示格式：两位小数 + 千分位（HALF_UP）
     */
    private String formatMoney(BigDecimal value) {
        return MONEY_FORMAT.format(value.setScale(2, RoundingMode.HALF_UP));
    }

    /**
     * 带符号的金额展示格式：正数前补「+」
     */
    private String formatMoneySigned(BigDecimal value) {
        BigDecimal rounded = value.setScale(2, RoundingMode.HALF_UP);
        if (rounded.signum() >= 0) {
            return "+" + MONEY_FORMAT.format(rounded);
        }
        return MONEY_FORMAT.format(rounded);
    }

    /**
     * 空值兜底为 0
     */
    private BigDecimal nz(BigDecimal value) {
        return value == null ? BigDecimal.ZERO : value;
    }
}
