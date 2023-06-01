import init, { run_bevy_app } from "yogurt-sim";

export async function init_bevy() {
  await init();
  run_bevy_app();
}
