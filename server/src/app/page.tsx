"use client";
import { init_bevy } from "@/utils/init_wasm";

export default function Home() {
  init_bevy();
  return (
    <div className="App">
      <canvas id="bevy_canvas"></canvas>
    </div>
  );
}
