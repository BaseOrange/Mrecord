package com.dcz.mrecord.config;

import org.apache.ibatis.type.BaseTypeHandler;
import org.apache.ibatis.type.JdbcType;
import org.apache.ibatis.type.MappedTypes;

import java.sql.CallableStatement;
import java.sql.PreparedStatement;
import java.sql.ResultSet;
import java.sql.SQLException;
import java.time.Instant;
import java.time.LocalDateTime;
import java.time.ZoneId;
import java.time.format.DateTimeFormatter;
import java.util.Date;

/**
 * SQLite日期类型处理器
 * <p>
 * 解决Druid连接池 + SQLite JDBC驱动组合下，java.util.Date被存储为epoch毫秒数
 * 而读取时驱动按日期字符串格式解析导致失败的问题。
 * <p>
 * 写入时将Date格式化为"yyyy-MM-dd HH:mm:ss"字符串存储，
 * 读取时支持解析日期字符串和epoch毫秒数两种格式，兼容历史数据。
 * <p>
 * 使用 {@link DateTimeFormatter}(线程安全、不可变)替代 SimpleDateFormat。
 *
 * @author dcz
 * @since 2026/05/21
 */
@MappedTypes(Date.class)
public class SqliteDateTypeHandler extends BaseTypeHandler<Date> {

    private static final DateTimeFormatter FORMATTER = DateTimeFormatter.ofPattern("yyyy-MM-dd HH:mm:ss");

    private static final ZoneId ZONE = ZoneId.systemDefault();

    /**
     * 写入参数时，将Date格式化为日期字符串
     */
    @Override
    public void setNonNullParameter(PreparedStatement ps, int i, Date parameter, JdbcType jdbcType) throws SQLException {
        LocalDateTime ldt = parameter.toInstant().atZone(ZONE).toLocalDateTime();
        ps.setString(i, FORMATTER.format(ldt));
    }

    /**
     * 按列名读取日期
     */
    @Override
    public Date getNullableResult(ResultSet rs, String columnName) throws SQLException {
        return parseDate(rs.getString(columnName));
    }

    /**
     * 按列索引读取日期
     */
    @Override
    public Date getNullableResult(ResultSet rs, int columnIndex) throws SQLException {
        return parseDate(rs.getString(columnIndex));
    }

    /**
     * 从存储过程读取日期
     */
    @Override
    public Date getNullableResult(CallableStatement cs, int columnIndex) throws SQLException {
        return parseDate(cs.getString(columnIndex));
    }

    /**
     * 解析日期字符串，兼容"yyyy-MM-dd HH:mm:ss"格式和epoch毫秒数格式
     *
     * @param value 数据库中存储的日期值
     * @return 解析后的Date对象，值为null或空时返回null
     */
    private Date parseDate(String value) {
        if (value == null || value.isEmpty()) {
            return null;
        }
        try {
            if (value.chars().allMatch(Character::isDigit)) {
                return new Date(Long.parseLong(value));
            }
            LocalDateTime ldt = LocalDateTime.parse(value, FORMATTER);
            return Date.from(ldt.atZone(ZONE).toInstant());
        } catch (Exception e) {
            throw new IllegalArgumentException("Failed to parse date: " + value, e);
        }
    }
}
