import { WebviewWindow } from "@tauri-apps/api/webviewWindow";
import { applyTranslations, bindLanguageMenu } from "./i18n";
import "./styles.css";

const helpMenuToggle = document.querySelector<HTMLButtonElement>("#help-menu-toggle")!;
const helpMenu = document.querySelector<HTMLElement>("#help-menu")!;
const helpMenuWrap = document.querySelector<HTMLElement>(".help-menu-wrap")!;
const backToMain = document.querySelector<HTMLButtonElement>("#back-to-main")!;
const languageToggle = document.querySelector<HTMLButtonElement>("#language-toggle")!;
const languageMenu = document.querySelector<HTMLElement>("#language-menu")!;

function setMenuOpen(open: boolean) {
  helpMenu.classList.toggle("hidden", !open);
  helpMenuToggle.setAttribute("aria-expanded", String(open));
}

helpMenuToggle.addEventListener("click", () => {
  setMenuOpen(helpMenu.classList.contains("hidden"));
});

languageToggle.addEventListener("click", () => {
  setMenuOpen(false);
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

bindLanguageMenu(languageToggle, languageMenu, () => applyTranslations(document));
