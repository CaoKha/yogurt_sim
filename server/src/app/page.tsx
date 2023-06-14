"use client";
import { init_wasm } from "./components/wasm/init";

export default function Home() {
  init_wasm();
  const doNothing = () => {
    console.log("do nothing!");
  };
  return (
    <>
      <h1 className="text-center font-bold font-mono text-lg ">
        A YOGURT SIMULATION{" "}
      </h1>
      <div style={{ height: "30rem", marginBottom: "3rem" }}>
        <h2 className="text-sky-500 font-mono font-bold"> 1. CANVAS </h2>
        <canvas id="bevy_canvas"></canvas>
      </div>
      <div>
        <h2 className="text-sky-500 font-mono font-bold"> 2. PROGRESS </h2>
        <ol>
          <li>
            <input type="checkbox" checked onChange={doNothing} />{" "}
            <span className="text-sm">Bevy</span>
          </li>

          <li>
            <input type="checkbox" checked={false} onChange={doNothing} />{" "}
            <span className="text-sm">Rapier</span>
          </li>
          <li>
            <input type="checkbox" checked={false} onChange={doNothing} />{" "}
            <span className="text-sm">Yogurt Asset</span>
          </li>
          <li>
            <input type="checkbox" checked={false} onChange={doNothing} />{" "}
            <span className="text-sm">Yogurt Simulation</span>
          </li>
        </ol>
      </div>
    </>
  );
}
