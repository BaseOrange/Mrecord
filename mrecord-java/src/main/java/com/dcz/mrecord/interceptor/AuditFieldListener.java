package com.dcz.mrecord.interceptor;

import com.dcz.mrecord.common.UserContext;
import com.dcz.mrecord.entity.BaseEntity;
import com.mybatisflex.annotation.InsertListener;
import com.mybatisflex.annotation.UpdateListener;
import lombok.extern.slf4j.Slf4j;
import org.springframework.stereotype.Component;

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
        }
    }

    /**
     * 获取当前操作人标识，优先取用户ID，其次取请求IP，均失败则返回"anonymous"
     *
     * @return 操作人标识
     */
    private String getIdOrIp() {
        String res = "anonymous";
        try {
            res = UserContext.getUserId();
        } catch (Exception e) {
            log.warn("获取当前用户ID失败", e);
            try {
                res = UserContext.getUserIp();
            } catch (Exception e1) {
                log.warn("获取当前用户IP失败", e1);
            }
        }

        return res;
    }
}
