import { createApp } from "vue";
import App from "./App.vue";
import "./styles.css";

const app = createApp(App);

// Surface render-time errors visibly — otherwise a thrown computed/template silently leaves
// the affected vnode subtree blank and the user sees a grey panel with no clue why.
app.config.errorHandler = (err, _instance, info) => {
  console.error(`[Vue] ${info}`, err);
  try {
    const box = document.createElement("div");
    box.style.cssText =
      "position:fixed;left:8px;right:8px;bottom:8px;z-index:99999;padding:10px 14px;" +
      "background:#1a0a0a;color:#ff6677;border:1px solid #ff007f;border-radius:4px;" +
      "font-family:monospace;font-size:12px;white-space:pre-wrap;max-height:40vh;overflow:auto;";
    box.textContent = `Render error in ${info}: ${err instanceof Error ? err.message : String(err)}`;
    document.body.appendChild(box);
  } catch { /* DOM not ready yet */ }
};

app.mount("#app");
