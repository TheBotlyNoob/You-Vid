import init, { App } from '/wasm/frontend.js';

init('/wasm/frontend_bg.wasm').then(() => new App());
