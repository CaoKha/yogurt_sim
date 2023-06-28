"use client";
import { init_wasm } from "./components/wasm/init";
import { CodeBlock, hopscotch } from "react-code-blocks";
import { initWorld, initWorldResources } from "./components/codeblocks";

export default function Home() {
  init_wasm();
  return (
    <>
      <h1 className="text-center font-bold font-mono text-lg ">
        A YOGURT SIMULATION{" "}
      </h1>
      <div style={{ height: "30rem", marginBottom: "3rem" }}>
        <h2 className="text-sky-500 font-mono font-bold"> I. CANVAS </h2>
        <canvas id="bevy_canvas"></canvas>
      </div>
      <div>
        <h2 className="text-sky-500 font-mono font-bold"> II. NOTES </h2>
        <ul className="list-disc">
          <li className="mb-5">
            <label>Init World</label>
            <CodeBlock
              text={initWorld}
              language="rust"
              theme={hopscotch}
              showLineNumbers={true}
            />
          </li>

          <li>
            <label>Init World Resources</label>
            <CodeBlock
              text={initWorldResources}
              language="rust"
              theme={hopscotch}
              showLineNumbers={true}
            />
          </li>
        </ul>
      </div>
    </>
  );
}
