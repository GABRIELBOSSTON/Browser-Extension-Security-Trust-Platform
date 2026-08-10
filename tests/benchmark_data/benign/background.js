chrome.runtime.onInstalled.addListener(() => {
    chrome.storage.local.set({ setup: true }, () => {
        console.log("Setup complete");
    });
});

async function fetchConfig() {
    const response = await fetch("https://api.example.com/config");
    const data = await response.json();
    console.log(data);
}
