import init, { run_bevy_example } from "./wasm/yogurt_sim";


export async function init_bevy() {
  await init();
  run_bevy_example();
}
