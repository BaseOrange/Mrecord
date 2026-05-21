package com.dcz.mrecord;

import org.mybatis.spring.annotation.MapperScan;
import org.springframework.boot.SpringApplication;
import org.springframework.boot.autoconfigure.SpringBootApplication;
import org.springframework.scheduling.annotation.EnableScheduling;

import java.nio.file.Files;
import java.nio.file.Path;

/**
 * 启动类
 *
 * @author dcz
 * @date 2026/04/03
 */
@SpringBootApplication
@EnableScheduling
@MapperScan("com.dcz.mrecord.mapper")
public class Application {
    public static void main(String[] args) throws Exception {
        Files.createDirectories(Path.of("./data"));
        SpringApplication.run(Application.class, args);
    }
}