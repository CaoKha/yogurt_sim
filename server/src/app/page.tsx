"use client";
import init, { run_bevy_app } from "yogurt-sim";

init();
export default function Home() {
  const runBevyApp = async () => {
    await init();
    run_bevy_app();
  };

  return (
    <div className="App">
      <button onClick={runBevyApp}>Run Bevy App</button>
    </div>
  );
}
