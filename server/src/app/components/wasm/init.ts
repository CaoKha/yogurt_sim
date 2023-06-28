import init, {
  InitOutput,
  // run_ball_game,
  // run_rapier2d_example,
  // run_rapier3d_example,
  // run_ocean_simple
  run_uv_sphere
} from "yogurt-sim";

export async function init_wasm() {
  await init()
    .then((wasm_js: InitOutput) => {
      console.log(wasm_js);
      run_uv_sphere();
      // run_ball_game();
      // run_rapier2d_example();
      // run_rapier3d_example();
    })
    .catch((_: unknown) => {
      // intentional error from wasm, we can ignore this error message:
      // "Using exceptions for control flow, don't mind me. This isn't actually an error ..."
      // console.error(e);
    });
}
