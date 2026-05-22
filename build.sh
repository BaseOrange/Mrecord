#!/bin/bash
set -e

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
VUE_DIR="$SCRIPT_DIR/mrecord-vue"
JAVA_DIR="$SCRIPT_DIR/mrecord-java"
STATIC_DIR="$JAVA_DIR/src/main/resources/static"

echo "=== Building Vue frontend ==="
cd "$VUE_DIR"
yarn install --frozen-lockfile
npx vite build

echo "=== Copying frontend to Spring Boot static resources ==="
rm -rf "$STATIC_DIR"
cp -r "$VUE_DIR/dist" "$STATIC_DIR"

echo "=== Building Spring Boot JAR ==="
cd "$JAVA_DIR"
export JAVA_HOME=$(/usr/libexec/java_home -v 17 2>/dev/null || echo "$JAVA_HOME")
mvn clean package -DskipTests

JAR_FILE=$(ls "$JAVA_DIR/target"/mrecord-*.jar 2>/dev/null | head -1)
echo ""
echo "=== Build complete ==="
echo "JAR: $JAR_FILE"
echo "Run: java -jar $JAR_FILE"
