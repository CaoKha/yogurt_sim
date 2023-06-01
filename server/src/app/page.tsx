"use client";
import { init_bevy } from "@/wasm/init";

export default function Home() {
  return (
    <div className="App">
      <button onClick={init_bevy}>Run Bevy App</button>
      <canvas id="bevy_canvas"></canvas>
    </div>
  );
}
