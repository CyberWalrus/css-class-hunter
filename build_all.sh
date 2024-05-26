#!/bin/bash
export PATH="/usr/local/opt/llvm/bin:$PATH"

targets=(
	"x86_64-pc-windows-gnu"
	"x86_64-apple-darwin"
	"aarch64-apple-darwin"
	"x86_64-unknown-linux-musl"
)

if ! command -v cargo &>/dev/null; then
	echo "Cargo не найден. Убедитесь, что Rust установлен и Cargo путь добавлен в PATH."
	exit 1
fi

for target in "${targets[@]}"; do
	echo "Building for target: $target"
	if ! cargo build --release --target $target; then
		echo "Build for $target failed"
		exit 1
	fi
done

echo "Build completed for all targets."
