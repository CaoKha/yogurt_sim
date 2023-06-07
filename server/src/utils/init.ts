import init, { run_ball_game } from "yogurt-sim";

export async function init_wasm() {
  await init().then(() => run_ball_game()).catch((e: unknown) => {
    console.log(e);
  });
}
