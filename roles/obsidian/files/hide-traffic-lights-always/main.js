const { Platform, Plugin } = require("obsidian");

module.exports = class HideTrafficLightsAlways extends Plugin {
  constructor(...args) {
    super(...args);
    this.windows = new Set();
  }

  onload() {
    if (!Platform.isMacOS) {
      return;
    }

    this.hideWindow(window);
    this.registerDomEvent(window, "focus", () => this.hideAll());
    this.registerEvent(this.app.workspace.on("layout-change", () => this.hideAll()));
    this.registerEvent(
      this.app.workspace.on("window-open", (_workspaceWindow, domWindow) => {
        this.hideWindow(domWindow);
      }),
    );
    this.registerEvent(
      this.app.workspace.on("window-close", (_workspaceWindow, domWindow) => {
        this.windows.delete(domWindow);
      }),
    );
  }

  onunload() {
    for (const domWindow of this.windows) {
      this.setWindowButtonPosition(domWindow, { x: 10, y: 16 });
    }
    this.windows.clear();
  }

  hideAll() {
    for (const domWindow of this.windows) {
      this.setWindowButtonPosition(domWindow, { x: -100, y: -100 });
    }
  }

  hideWindow(domWindow) {
    this.windows.add(domWindow);
    this.setWindowButtonPosition(domWindow, { x: -100, y: -100 });
  }

  setWindowButtonPosition(domWindow, position) {
    try {
      const electron =
        domWindow === window ? require("electron") : domWindow.require?.("electron");
      electron?.remote?.getCurrentWindow()?.setWindowButtonPosition(position);
    } catch (error) {
      console.error("Failed to update macOS traffic-light buttons:", error);
    }
  }
};
