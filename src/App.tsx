import { useRef } from "react";
import "./App.css";
import { vec3, mag, sub } from "./utils/math";
import { rgb } from "./utils/colour";
import type { WorkerInMessage, SceneObject, WorkerSettings, ModelData } from "./render.types";
import RABBIT_MODEL from "./models/rabbit";

interface Settings extends WorkerSettings {
  gamma: number;
}

const FOCAL_LENGTH = 550;
const FOCAL_DISTANCE = 150;
const APERTURE = 0.1;

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
    transmission: 0.0,
    ior: 1.5,
  },
  // center
  {
    shape: "sphere",
    radius: MAIN_SIZE,
    position: vec3(0, 0, MAIN_Z),
    emission: rgb(0, 0, 0),
    albedo: rgb(0.9, 0.9, 0.9),
    metallic: 0.0,
    roughness: 0,
    transmission: 1.0,
    ior: 1.5,
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
    transmission: 0.0,
    ior: 1.5,
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
    transmission: 0.0,
    ior: 1.5,
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
    transmission: 0.0,
    ior: 1.5,
  },
];

for (let i = 0; i < 250; i++) {
  for (let attempts = 0; attempts < 100; attempts++) {
    const radius = 2 + Math.random() * 5;
    const x = (Math.random() - 0.5) * 150;
    const z = MAIN_Z + (Math.random() - 0.5) * 175;
    const position = vec3(x, MAIN_SIZE - radius, z);

    let isIntersecting = false;
    for (const obj of SCENE_DATA) {
      if (obj.shape === "sphere") {
        if (mag(sub(position, obj.position)) < radius + obj.radius) {
          isIntersecting = true;
          break;
        }
      }
    }

    if (!isIntersecting) {
      const metallic = Math.random() > 0.6 ? 1.0 : 0.0;
      const transmission = !metallic && Math.random() > 0.7 ? 1.0 : 0.0;

      SCENE_DATA.push({
        shape: "sphere",
        radius,
        position,
        emission: rgb(0, 0, 0),
        albedo: rgb(Math.random(), Math.random(), Math.random()),
        metallic,
        roughness: Math.random(),
        transmission,
        ior: 1.5,
      });

      break;
    }
  }
}

const MODELS: ModelData[] = [
  {
    obj: RABBIT_MODEL,
    position: vec3(15, 34, 100),
    rotation: vec3(Math.PI, 0, 0),
    scale: 250,
    emission: rgb(0, 0, 0),
    albedo: rgb(0.5, 0.5, 0.5),
    metallic: 0,
    roughness: 0.5,
    transmission: 0,
    ior: 1.5,
  },
];

function startRender(canvas: HTMLCanvasElement): Worker {
  const worker = new Worker(new URL("./renderer.worker.ts", import.meta.url));
  const offscreen = canvas.transferControlToOffscreen();
  worker.onerror = (e) => console.error("worker error:", e);

  const msg: WorkerInMessage = {
    type: "start",
    canvas: offscreen,
    settings: SETTINGS,
    entities: SCENE_DATA,
    models: MODELS,
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
