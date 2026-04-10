package com.dcz.mrecord.util;

import io.jsonwebtoken.Claims;
import io.jsonwebtoken.Jwts;
import io.jsonwebtoken.security.Keys;
import org.springframework.stereotype.Component;

import javax.crypto.SecretKey;
import java.nio.charset.StandardCharsets;
import java.util.Date;

/**
 * JWT工具类
 *
 * @author dcz
 * @since 2026/04/10
 */
@Component
public class JwtUtil {

    /**
     * 密钥
     * TODO 后续考虑移入配置项中
     */
    private static final String SECRET = "156e578e-7727-4951-afb2-1b77ad6a0497";

    /**
     * 过期时间 7天
     * TODO 后续考虑移入配置项中
     */
    private static final long EXPIRE = 7 * 24 * 60 * 60 * 1000L;

    private final SecretKey key = Keys.hmacShaKeyFor(SECRET.getBytes(StandardCharsets.UTF_8));

    /**
     * 生成 token，只存 userId
     */
    public String createToken(String userId) {
        if (userId == null || userId.isEmpty()) {
            return null;
        }

        return Jwts.builder().subject(userId).expiration(new Date(System.currentTimeMillis() + EXPIRE)).signWith(key).compact();
    }

    /**
     * 解析 userId
     */
    public String getUserId(String token) {
        try {
            Claims claims = Jwts.parser().verifyWith(key).build().parseSignedClaims(token).getPayload();
            return claims.getSubject();
        } catch (Exception e) {
            return null;
        }
    }
}
