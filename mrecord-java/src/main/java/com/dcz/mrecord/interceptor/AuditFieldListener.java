package com.dcz.mrecord.interceptor;

import com.dcz.mrecord.common.UserContext;
import com.dcz.mrecord.entity.BaseEntity;
import com.mybatisflex.annotation.InsertListener;
import com.mybatisflex.annotation.UpdateListener;
import org.springframework.stereotype.Component;

/**
 * 审计字段监听器
 *
 * @author dcz
 * @since 2023/4/16 08:44
 */
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
            String userId = UserContext.getUserId();

            // 防空（未登录场景）
            if (userId != null) {
                base.setCreateBy(userId);
                base.setUpdateBy(userId);
            }
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
            String userId = UserContext.getUserId();

            if (userId != null) {
                base.setUpdateBy(userId);
            }
        }
    }
}
