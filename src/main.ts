import { invoke } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";

let checkBtn: HTMLButtonElement | null;
let setupBtn: HTMLButtonElement | null;
let folderBtn: HTMLButtonElement | null;
let liveBtn: HTMLButtonElement | null;
let deadBtn: HTMLButtonElement | null;
let statusText: HTMLElement | null;
let linkContainer: HTMLElement | null;

async function checkDocker() {
  if (!statusText) return;
  statusText.textContent = "Checking Docker engine...";
  statusText.className = "loading";

  try {
    await invoke<string>("check_docker");
    statusText.innerHTML = "🟢 <b>System Ready!</b> Docker is running.";
    statusText.className = "success";
  } catch (error) {
    statusText.innerHTML = `🔴 <b>Error:</b> ${error}`;
    statusText.className = "error";
  }
}

async function setupFolder() {
  if (!statusText) return;
  statusText.textContent =
    "Downloading Immich files from GitHub... Please wait.";
  statusText.className = "loading";

  try {
    const response = await invoke<string>("setup_immich_folder");
    statusText.innerHTML = `🟢 <b>Success!</b> ${response}`;
    statusText.className = "success";
  } catch (error) {
    statusText.innerHTML = `🔴 <b>Download Error:</b> ${error}`;
    statusText.className = "error";
  }
}

async function chooseFolder() {
  if (!statusText) return;

  try {
    // 1. Open the native OS folder picker
    const selectedPath = await open({
      directory: true, // We want a folder, not a file
      multiple: false,
      title: "Select your Immich Photos folder",
    });

    if (selectedPath) {
      statusText.textContent = `Configuring server to use: ${selectedPath}...`;
      statusText.className = "loading";

      // 2. Send the path to our Rust surgeon
      const response = await invoke<string>("update_env_file", {
        uploadPath: selectedPath,
      });

      statusText.innerHTML = `🟢 <b>Success!</b> ${response}`;
      statusText.className = "success";
    } else {
      statusText.innerHTML = `⚪ Folder selection cancelled.`;
      statusText.className = "";
    }
  } catch (error) {
    statusText.innerHTML = `🔴 <b>Error:</b> ${error}`;
    statusText.className = "error";
  }
}

async function goLive() {
  if (!statusText || !linkContainer) return;
  statusText.textContent =
    "Starting Immich Engine... (This might take a minute on the first run)";
  statusText.className = "loading";
  linkContainer.innerHTML = "";

  try {
    const response = await invoke<string>("start_server");
    statusText.innerHTML = `<b>Success:</b> ${response}`;
    statusText.className = "success";
    linkContainer.innerHTML = `<a href="http://localhost:2283" target="_blank" style="color: #89b4fa;">Open Immich Web App: http://localhost:2283</a>`;
  } catch (error) {
    statusText.innerHTML = `🔴 <b>Error:</b> ${error}`;
    statusText.className = "error";
  }
}

async function goDead() {
  if (!statusText || !linkContainer) return;
  statusText.textContent = "Shutting down database and server gracefully...";
  statusText.className = "loading";
  linkContainer.innerHTML = "";

  try {
    const response = await invoke<string>("stop_server");
    statusText.innerHTML = `<b>Status:</b> ${response}`;
    statusText.className = "success";
  } catch (error) {
    statusText.innerHTML = `🔴 <b>Error:</b> ${error}`;
    statusText.className = "error";
  }
}

async function checkInitialStatus() {
  if (!statusText || !linkContainer) return;

  try {
    const isLive = await invoke<boolean>("check_server_status");

    if (isLive) {
      statusText.innerHTML = "<b>Status:</b> Server is currently LIVE! 🟢";
      statusText.className = "success";
      linkContainer.innerHTML = `<a href="http://localhost:2283" target="_blank" style="color: #89b4fa;">Open Immich Web App: http://localhost:2283</a>`;
    } else {
      statusText.innerHTML = "<b>Status:</b> Server is currently OFFLINE. ⚪";
      statusText.className = "";
    }
  } catch (error) {
    console.error("Failed to check server status:", error);
  }
}

window.addEventListener("DOMContentLoaded", () => {
  checkBtn = document.querySelector("#check-btn");
  setupBtn = document.querySelector("#setup-btn");
  folderBtn = document.querySelector("#folder-btn");
  liveBtn = document.querySelector("#live-btn");
  deadBtn = document.querySelector("#dead-btn");
  statusText = document.querySelector("#status");
  linkContainer = document.querySelector("#link-container");

  if (checkBtn) checkBtn.addEventListener("click", () => checkDocker());
  if (setupBtn) setupBtn.addEventListener("click", () => setupFolder());
  if (folderBtn) folderBtn.addEventListener("click", () => chooseFolder());
  if (liveBtn) liveBtn.addEventListener("click", () => goLive());
  if (deadBtn) deadBtn.addEventListener("click", () => goDead());
  checkInitialStatus();
});
