import init, { run_app } from '/wasm/you_vid_frontend.js';

init('/wasm/you_vid_frontend_bg.wasm').then(run_app);
