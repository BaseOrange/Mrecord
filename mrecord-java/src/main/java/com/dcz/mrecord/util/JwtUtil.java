package com.dcz.mrecord.util;

import com.dcz.mrecord.config.MrConf;
import io.jsonwebtoken.Claims;
import io.jsonwebtoken.Jwts;
import io.jsonwebtoken.security.Keys;
import jakarta.annotation.Resource;
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

    @Resource
    private MrConf mrConf;

    /**
     * 生成 token，只存 userId
     * @param userId userId
     * @return  token
     */
    public String createToken(String userId) {
        if (userId == null || userId.isEmpty()) {
            return null;
        }

        SecretKey key = Keys.hmacShaKeyFor(mrConf.getJwtSecret().getBytes(StandardCharsets.UTF_8));
        return Jwts.builder().subject(userId).expiration(new Date(System.currentTimeMillis() + mrConf.getJwtExpire())).signWith(key).compact();
    }

    /**
     * 解析 userId
     * @param token  token
     * @return userId
     */
    public String getUserId(String token) {
        try {
            SecretKey key = Keys.hmacShaKeyFor(mrConf.getJwtSecret().getBytes(StandardCharsets.UTF_8));
            Claims claims = Jwts.parser().verifyWith(key).build().parseSignedClaims(token).getPayload();
            return claims.getSubject();
        } catch (Exception e) {
            return null;
        }
    }
}
