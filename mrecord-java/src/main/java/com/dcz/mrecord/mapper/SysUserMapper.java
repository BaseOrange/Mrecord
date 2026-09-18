package com.dcz.mrecord.mapper;

import com.dcz.mrecord.entity.SysUser;
import com.mybatisflex.core.BaseMapper;
import org.apache.ibatis.annotations.Delete;
import org.apache.ibatis.annotations.Mapper;
import org.apache.ibatis.annotations.Param;

/**
 * 用户Mapper
 *
 * @author dcz
 * @since 2026/04/07
 */
@Mapper
public interface SysUserMapper extends BaseMapper<SysUser> {

    /**
     * 物理删除用户
     *
     * <p>MyBatis-Flex 对 {@code MR_IS_DELETED} 配置了逻辑删除，{@link BaseMapper#deleteById}
     * 只会置位逻辑删除标识；注销清理需要真正移除用户行，故用原生 DELETE 绕过逻辑删除。</p>
     *
     * @param userId 用户ID
     * @return 受影响行数
     */
    @Delete("DELETE FROM SYS_USER WHERE MR_ID = #{userId}")
    int physicalDeleteById(@Param("userId") String userId);
}
