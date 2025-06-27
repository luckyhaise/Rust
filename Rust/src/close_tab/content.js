document.addEventListener('visibilitychange', () => {
  if (document.visibilityState !== "hidden") return;

  const overlay = document.createElement("div");
  overlay.style = `
    position: fixed;
    top: 0; left: 0; width: 100%; height: 100%;
    background: rgba(0,0,0,0.85);
    color: white;
    font-size: 2rem;
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 999999;
  `;
  overlay.textContent = "Close this tab? (Enter = Yes, Esc = No)";
  document.body.appendChild(overlay);

  function cleanup() {
    document.body.removeChild(overlay);
    window.removeEventListener("keydown", onKeydown);
  }

  function onKeydown(e) {
    if (e.key === "Enter") {
      cleanup();
      chrome.runtime.sendMessage({ action: "close_this_tab" });
    } else if (e.key === "Escape") {
      cleanup();
    }
  }

  window.addEventListener("keydown", onKeydown);
});
