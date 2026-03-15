# 📸 Immich Server Manager

A lightweight, cross-platform desktop companion app for managing a self-hosted [Immich](https://immich.app/) photo server.

Built specifically for the "Offline Vault" workflow: keep your massive photo library safely stored on an external hard drive, plug it in once a month, and spin up the Immich server with a single click to sync your mobile devices.

![Platform: Windows | macOS | Linux](https://img.shields.io/badge/Platform-Windows%20%7C%20macOS%20%7C%20Linux-blue)
![Built with Tauri & Rust](https://img.shields.io/badge/Built_with-Tauri_%7C_Rust-f38ba8)

## ✨ Features

- **1-Click Install:** Automatically downloads the official Docker configurations directly from the Immich repository. No terminal required.
- **Native Folder Picker:** Dynamically map your Immich `UPLOAD_LOCATION` and Database directories to an external hard drive (or anywhere else) using your OS's native file explorer.
- **Safe Ignition Switch:** Start and safely stop your Docker containers with a click. Prevents database corruption when unmounting external drives.
- **Background Daemon (System Tray):** Close the window, and the app seamlessly minimizes to your taskbar/menu bar while your server continues to run.
- **Auto-Status Detection:** Instantly knows if your server is offline or live the moment you open the app.
- **Ultra-Lightweight:** Built with Tauri v2 (Rust). The app size is under 10MB and idles with virtually zero RAM usage.

## 📥 Download & Installation

1. Head over to the [Releases Page](../../releases/latest).
2. Download the installer for your operating system:
   - **Windows:** `.exe` or `.msi`
   - **macOS:** `.dmg` (Apple Silicon or Intel)
   - **Linux:** `.AppImage` or `.deb`
3. **Prerequisite:** You must have [Docker Desktop](https://www.docker.com/products/docker-desktop/) (or the Docker Engine on Linux) installed and running before using this app.

## 🚀 The "Plug & Sync" Workflow

This app was designed for users who don't want to run a NAS 24/7.

1. Plug your portable SSD/HDD into your computer.
2. Open **Immich Server Manager**.
3. If it's your first time, click **Download Server Files** and select your external drive as the **Photo Folder**.
4. Click **GO LIVE 🚀**.
5. Open the Immich app on your phone to instantly sync your latest photos over your local Wi-Fi.
6. When finished, click **GO DEAD 🛑** to safely spin down the database, and unplug your drive.

## 🛠️ Development

If you want to compile this app yourself:

```bash
# Clone the repository
git clone [https://github.com/YOUR_USERNAME/immich-server-manager.git](https://github.com/YOUR_USERNAME/immich-server-manager.git)
cd immich-server-manager

# Install dependencies
pnpm install

# Run in development mode
pnpm tauri dev

# Build for release
pnpm tauri build
```
