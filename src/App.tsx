import { useRef } from "react";
import "./App.css";
import { vec3 } from "./utils/math";
import { rgb } from "./utils/colour";
import type { WorkerInMessage, SceneObject, WorkerSettings } from "./render.types";

interface Settings extends WorkerSettings {
  gamma: number;
}

const FOCAL_LENGTH = 1000;
const FOCAL_DISTANCE = FOCAL_LENGTH / 4;
const APERTURE = FOCAL_DISTANCE / 200;

const SETTINGS: Settings = {
  width: 640,
  height: 480,
  focalLength: FOCAL_LENGTH,
  focalDistance: FOCAL_DISTANCE,
  aperture: APERTURE,
  samples: 500,
  bounces: 50,
  fov: 80,
  gamma: 2.2,
};

const MAIN_Z: number = SETTINGS.focalDistance;
const MAIN_SIZE: number = 25;

const SCENE_DATA: SceneObject[] = [
  // floor
  {
    shape: "plane",
    position: vec3(0, MAIN_SIZE, 0),
    normal: vec3(0, -1, 0),
    emission: rgb(0, 0, 0),
    albedo: rgb(0.5, 0.5, 0.5),
    metallic: 0.0,
    roughness: 1.0,
  },
  // center
  {
    shape: "sphere",
    radius: MAIN_SIZE,
    position: vec3(0, 0, MAIN_Z),
    emission: rgb(0, 0, 0),
    albedo: rgb(0.5, 0.5, 0.5),
    metallic: 0.0,
    roughness: 1.0,
  },
  // red light
  {
    shape: "sphere",
    radius: MAIN_SIZE,
    position: vec3(MAIN_SIZE * 2.5, 0, MAIN_Z),
    emission: rgb(768, 0, 0),
    albedo: rgb(1.0, 0.0, 0.0),
    metallic: 0.0,
    roughness: 1.0,
  },
  // back
  {
    shape: "sphere",
    radius: MAIN_SIZE,
    position: vec3(MAIN_SIZE * 2.5 * 0.6, 0, MAIN_Z + MAIN_SIZE * 2),
    emission: rgb(0, 0, 0),
    albedo: rgb(0.6, 0.92, 0.2),
    metallic: 1.0,
    roughness: 0.1,
  },
  // forword
  {
    shape: "sphere",
    radius: MAIN_SIZE,
    position: vec3(-(MAIN_SIZE * 2.5 * 0.6), 0, MAIN_Z - MAIN_SIZE * 1),
    emission: rgb(0, 0, 0),
    albedo: rgb(0.1, 0.3, 1.0),
    metallic: 0.0,
    roughness: 0.2,
  },
];

for (let i = 0; i < 25; i++) {
  const radius = 2 + Math.random() * 5;
  const x = (Math.random() - 0.5) * 200;
  const z = MAIN_Z + (Math.random() - 0.5) * 200;
  const metallic = Math.random() > 0.6 ? 1.0 : 0.0;
  SCENE_DATA.push({
    shape: "sphere",
    radius,
    position: vec3(x, MAIN_SIZE - radius, z),
    emission: rgb(0, 0, 0),
    albedo: rgb(Math.random(), Math.random(), Math.random()),
    metallic,
    roughness: Math.random(),
  });
}

function startRender(canvas: HTMLCanvasElement): Worker {
  const worker = new Worker(new URL("./renderer.worker.ts", import.meta.url));
  const offscreen = canvas.transferControlToOffscreen();
  worker.onerror = (e) => console.error("worker error:", e);

  const msg: WorkerInMessage = {
    type: "start",
    canvas: offscreen,
    settings: SETTINGS,
    entities: SCENE_DATA,
    gamma: SETTINGS.gamma,
  };
  worker.postMessage(msg, [offscreen]);

  return worker;
}

function App() {
  const workerRef = useRef<Worker | null>(null);

  const canvasRefCallback = (canvas: HTMLCanvasElement | null) => {
    if (!canvas) return;
    if (workerRef.current) {
      workerRef.current.terminate();
      workerRef.current = null;
    }
    workerRef.current = startRender(canvas);
  };

  return (
    <div className="App">
      <canvas ref={canvasRefCallback} width={SETTINGS.width} height={SETTINGS.height} />
    </div>
  );
}

export default App;
