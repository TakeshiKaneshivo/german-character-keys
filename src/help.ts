import { WebviewWindow } from "@tauri-apps/api/webviewWindow";
import "./styles.css";

const helpMenuToggle = document.querySelector<HTMLButtonElement>("#help-menu-toggle")!;
const helpMenu = document.querySelector<HTMLElement>("#help-menu")!;
const helpMenuWrap = document.querySelector<HTMLElement>(".help-menu-wrap")!;
const backToMain = document.querySelector<HTMLButtonElement>("#back-to-main")!;

function setMenuOpen(open: boolean) {
  helpMenu.classList.toggle("hidden", !open);
  helpMenuToggle.setAttribute("aria-expanded", String(open));
}

helpMenuToggle.addEventListener("click", () => {
  setMenuOpen(helpMenu.classList.contains("hidden"));
});

document.addEventListener("click", (event) => {
  if (!helpMenuWrap.contains(event.target as Node)) setMenuOpen(false);
});

document.addEventListener("keydown", (event) => {
  if (event.key === "Escape") setMenuOpen(false);
});

backToMain.addEventListener("click", async () => {
  setMenuOpen(false);
  const mainWindow = await WebviewWindow.getByLabel("main");
  if (mainWindow) {
    await mainWindow.unminimize();
    await mainWindow.show();
    await mainWindow.setFocus();
  }
  await WebviewWindow.getCurrent().close();
});
