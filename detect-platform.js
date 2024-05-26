#!/usr/bin/env node

const { execSync } = require('child_process');
const os = require('os');

const platform = os.platform();
const arch = os.arch();

let binary = '';

if (platform === 'win32') {
    if (arch === 'x64') {
        binary = './bin/css-class-hunter-x86_64-pc-windows-gnu.exe';
    } else {
        console.error('Unsupported Windows architecture');
        process.exit(1);
    }
} else if (platform === 'linux') {
    if (arch === 'x64') {
        binary = './bin/css-class-hunter-x86_64-unknown-linux-musl';
    } else {
        console.error('Unsupported Linux architecture');
        process.exit(1);
    }
} else if (platform === 'darwin') {
    if (arch === 'x64') {
        binary = './bin/css-class-hunter-x86_64-apple-darwin';
    } else if (arch === 'arm64') {
        binary = './bin/css-class-hunter-aarch64-apple-darwin';
    } else {
        console.error('Unsupported macOS architecture');
        process.exit(1);
    }
} else {
    console.error('Unsupported platform');
    process.exit(1);
}

try {
    execSync(binary, { stdio: 'inherit' });
} catch (error) {
    console.error(`Failed to execute binary: ${binary}`);
    process.exit(1);
}
