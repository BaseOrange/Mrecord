# 使用Zulu JDK 17 基础镜像
FROM azul/zulu-openjdk-alpine:17

# 设置工作目录为 /app
WORKDIR /app

# 将本地编译好的 jar 包复制到容器中，并重命名为 app.jar
# （文件名对应 mrecord-java/pom.xml 中的 <version>1.0.0</version>）
COPY ./mrecord-java/target/mrecord-1.0.0.jar ./mrecord.jar

# 声明数据卷，用于持久化存储 SQLite 数据库文件
VOLUME ["/app/data"]

# 暴露应用端口
EXPOSE 2333

# 定义容器启动命令
ENTRYPOINT ["java", "-jar", "mrecord.jar"]