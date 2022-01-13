import init, { run_app } from '/wasm/frontend.js';

init('/wasm/frontend_bg.wasm').then(run_app);
