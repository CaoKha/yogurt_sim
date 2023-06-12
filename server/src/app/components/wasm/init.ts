import init, { InitOutput, run_lighting_example } from "yogurt-sim";

export async function init_wasm() {
  await init()
    .then((wasm_js: InitOutput) => {
      console.log(wasm_js);
      run_lighting_example();
    })
    .catch((e: unknown) => {
      // intentional error from wasm, we can ignore this error message:
      // "Using exceptions for control flow, don't mind me. This isn't actually an error ..."
      // console.error(e);
    });
}
