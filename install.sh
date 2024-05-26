#!/bin/bash
set -e

# Проверяет установку пакета и устанавливает его, если он не установлен
install_if_not_installed() {
	if ! brew list "$1" &>/dev/null; then
		echo "Installing $1..."
		brew install "$1"
	else
		echo "$1 is already installed"
	fi
}

# Установка Homebrew если он не установлен
if ! command -v brew &>/dev/null; then
	echo "Homebrew не найден. Устанавливаю..."
	/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
else
	echo "Homebrew уже установлен"
fi

# Обновление и перезапуск brew
brew update

# Установка необходимых пакетов через brew
install_if_not_installed gcc
install_if_not_installed llvm
install_if_not_installed x86_64-unknown-linux-gnu
install_if_not_installed arm-linux-gnueabihf-binutils
install_if_not_installed mingw-w64
install_if_not_installed messense/macos-cross-toolchains/aarch64-unknown-linux-gnu
install_if_not_installed messense/macos-cross-toolchains/armv7-unknown-linux-gnueabihf

# Установка rustup и нужных компонентов, если он не установлен
if ! command -v rustup &>/dev/null; then
	echo "rustup не найден. Устанавливаю..."
	curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
	source $HOME/.cargo/env
else
	echo "rustup уже установлен"
fi

# Установка целевых платформ для Rust
rustup target add aarch64-unknown-linux-gnu
rustup target add armv7-unknown-linux-gnueabihf
rustup target add x86_64-unknown-linux-gnu
rustup target add x86_64-pc-windows-gnu
rustup target add x86_64-apple-darwin
rustup target add aarch64-apple-darwin
rustup target add x86_64-unknown-linux-musl

echo "Все необходимые пакеты и целевые платформы установлены!"
