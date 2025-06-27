chrome.runtime.onMessage.addListener((msg, sender) => {
  console.log("📩 Received message:", msg);
  if (msg.action === "close_this_tab" && sender.tab?.id) {
    chrome.tabs.remove(sender.tab.id);
  }
});
