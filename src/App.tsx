import { useCallback, useEffect, useState } from "react";
import "./App.css";
import Canvas from "./Canvas";
import { Vec3, vec3 } from "./utils/math";
import { RGB, rgb } from "./utils/colour";
import initWASM, { Scene, Entity, Vec3 as WasmVec3, Rgb as WasmRGB } from "wasm-lib";

interface BaseObject {
  position: Vec3;
  emission: RGB;
  albedo: RGB;
  metallic: number;
  roughness: number;
  transparency: number;
  ior: number;
}

interface Sphere extends BaseObject {
  shape: "sphere";
  radius: number;
}

type SceneObject = Sphere;

interface Settings {
  width: number;
  height: number;
  focalLength: number;
  samples: number;
  bounces: number;
  fov: number;
}

const SETTINGS: Settings = {
  width: 640,
  height: 480,
  focalLength: 1000,
  samples: 500,
  bounces: 50,
  fov: 80,
};

const MAIN_Z: number = SETTINGS.focalLength / 4;
const MAIN_SIZE: number = 25;
const FLOOR_SIZE: number = 5000;

const SCENE_DATA: SceneObject[] = [
  // floor
  {
    shape: "sphere",
    position: vec3(0, FLOOR_SIZE + MAIN_SIZE, MAIN_Z),
    radius: FLOOR_SIZE,
    emission: rgb(0, 0, 0),
    albedo: rgb(0.5, 0.5, 0.5),
    metallic: 0.0,
    roughness: 1.0,
    transparency: 0,
    ior: 1.5,
  },
  // center - glass sphere
  {
    shape: "sphere",
    radius: MAIN_SIZE,
    position: vec3(0, 0, MAIN_Z),
    emission: rgb(0, 0, 0),
    albedo: rgb(1.0, 1.0, 1.0),
    metallic: 0.0,
    roughness: 0.0,
    transparency: 1.0,
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
    transparency: 0,
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
    transparency: 0,
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
    transparency: 0,
    ior: 1.5,
  },
];

const FLOOR_CENTER_Y = FLOOR_SIZE + MAIN_SIZE;
const FLOOR_CENTER_Z = MAIN_Z;

for (let i = 0; i < 25; i++) {
  const radius = 2 + Math.random() * 5;
  const x = (Math.random() - 0.5) * 200;
  const z = MAIN_Z + (Math.random() - 0.5) * 200;
  const dx = x;
  const dz = z - FLOOR_CENTER_Z;
  const surfaceY = FLOOR_CENTER_Y - Math.sqrt(FLOOR_SIZE * FLOOR_SIZE - dx * dx - dz * dz);
  const materialRoll = Math.random();
  const metallic = materialRoll > 0.6 && materialRoll <= 0.85 ? 1.0 : 0.0;
  const transparency = materialRoll > 0.85 ? 1.0 : 0.0;
  SCENE_DATA.push({
    shape: "sphere",
    radius,
    position: vec3(x, surfaceY - radius, z),
    emission: rgb(0, 0, 0),
    albedo: transparency > 0 ? rgb(1.0, 1.0, 1.0) : rgb(Math.random(), Math.random(), Math.random()),
    metallic,
    roughness: transparency > 0 ? 0.0 : Math.random(),
    transparency,
    ior: 1.5,
  });
}

function App() {
  const [context, setContext] = useState<CanvasRenderingContext2D | null>(null);

  useEffect(() => {
    if (!context) return;

    const renderScene = async () => {
      await initWASM();

      const scene = new Scene(
        context.canvas.width,
        context.canvas.height,
        SETTINGS.focalLength,
        SETTINGS.samples,
        SETTINGS.bounces,
        SETTINGS.fov,
      );

      SCENE_DATA.forEach((obj) => {
        const entity = new Entity(
          new WasmVec3(obj.position.x, obj.position.y, obj.position.z),
          new WasmRGB(obj.emission.r, obj.emission.g, obj.emission.b),
          new WasmRGB(obj.albedo.r, obj.albedo.g, obj.albedo.b),
          obj.metallic,
          obj.roughness,
          obj.radius,
          obj.transparency,
          obj.ior,
        );
        scene.add_entity(entity);
      });

      scene.set_gamma_correction(2.2);
      scene.render(context);
    };

    renderScene();
  }, [context]);

  const onCanvasInit = useCallback((ctx: CanvasRenderingContext2D) => {
    setContext(ctx);
  }, []);

  return (
    <div className="App">
      <Canvas animating={false} width={SETTINGS.width} height={SETTINGS.height} init={onCanvasInit} frame={() => {}} />
    </div>
  );
}

export default App;
