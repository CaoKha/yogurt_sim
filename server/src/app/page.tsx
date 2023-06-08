"use client";
import { init_wasm } from "./components/wasm/init";

export default function Home() {
  init_wasm();
  return (
    <div className="App">
      <canvas id="bevy_canvas"></canvas>
    </div>
  );
}

