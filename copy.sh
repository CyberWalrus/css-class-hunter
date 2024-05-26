#!/bin/sh

# Директории
BIN_DIR="bin"

# Платформы и соответствующие исходные и целевые файлы
FILES_TO_COPY="
target/x86_64-pc-windows-gnu/release/css-class-hunter.exe bin/css-class-hunter-x86_64-pc-windows-gnu.exe
target/aarch64-apple-darwin/release/css-class-hunter bin/css-class-hunter-aarch64-apple-darwin
target/x86_64-apple-darwin/release/css-class-hunter bin/css-class-hunter-x86_64-apple-darwin
target/x86_64-unknown-linux-musl/release/css-class-hunter bin/css-class-hunter-x86_64-unknown-linux-musl
"

# Создание целевой директории, если она не существует
if [ ! -d "$BIN_DIR" ]; then
	mkdir -p "$BIN_DIR"
fi

# Копирование файлов
echo "$FILES_TO_COPY" | while read -r SOURCE_FILE TARGET_FILE; do
	# Проверка существования исходного файла
	if [ ! -f "$SOURCE_FILE" ]; then
		echo "Исходный файл не найден: $SOURCE_FILE"
		continue
	fi

	# Копирование файла
	cp "$SOURCE_FILE" "$TARGET_FILE"

	# Проверка успешности копирования
	if [ $? -eq 0 ]; then
		echo "Файл успешно скопирован в $TARGET_FILE"
	else
		echo "Ошибка копирования файла: $SOURCE_FILE"
	fi
done
