package com.dcz.mrecord.interceptor;

import com.dcz.mrecord.common.UserContext;
import com.dcz.mrecord.entity.BaseEntity;
import com.mybatisflex.annotation.InsertListener;
import com.mybatisflex.annotation.UpdateListener;
import lombok.extern.slf4j.Slf4j;
import org.springframework.stereotype.Component;

import java.util.Date;

/**
 * 审计字段监听器
 *
 * @author dcz
 * @since 2023/4/16 08:44
 */
@Slf4j
@Component
public class AuditFieldListener implements InsertListener, UpdateListener {

    /**
     * 插入时执行
     *
     * @param entity 实体对象
     */
    @Override
    public void onInsert(Object entity) {
        if (entity instanceof BaseEntity) {
            BaseEntity base = (BaseEntity) entity;
            String operator = getIdOrIp();
            base.setCreateBy(operator);
            base.setUpdateBy(operator);
            Date now = new Date();
            base.setCreateTime(now);
            base.setUpdateTime(now);
        }
    }

    /**
     * 更新时执行
     *
     * @param entity 实体对象
     */
    @Override
    public void onUpdate(Object entity) {
        if (entity instanceof BaseEntity) {
            BaseEntity base = (BaseEntity) entity;
            base.setUpdateBy(getIdOrIp());
            base.setUpdateTime(new Date());
        }
    }

    /**
     * 获取当前操作人标识，优先取用户ID，其次取请求IP，均失败则返回"anonymous"
     *
     * @return 操作人标识
     */
    private String getIdOrIp() {
        String userId = UserContext.getUserId();
        if (userId != null && !userId.isBlank()) {
            return userId;
        }
        String userIp = UserContext.getUserIp();
        log.info("未获取到用户ID，记录操作IP: {}", userIp);
        return userIp;
    }
}
