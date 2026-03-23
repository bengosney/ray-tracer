import { useCallback, useEffect, useState } from "react";
import "./App.css";
import Canvas from "./Canvas";
import { Vec3, vec3 } from "./utils/math";
import { RGB, rgb } from "./utils/colour";
import initWASM, { Scene, Entity, Vec3 as WasmVec3, Rgb as WasmRGB } from "wasm-lib";

interface BaseObject {
  position: Vec3;
  emission: RGB;
  reflectivity: RGB;
  roughness: number;
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
  samples: 5,
  bounces: 50,
  fov: 80,
};

const MAIN_Z: number = SETTINGS.focalLength / 4;
const MAIN_SIZE: number = 25;
const FLOOR_SIZE: number = 5000;

const SCENE_DATA: SceneObject[] = [
  {
    shape: "sphere",
    position: vec3(0, FLOOR_SIZE + MAIN_SIZE, MAIN_Z),
    radius: FLOOR_SIZE,
    emission: rgb(0, 0, 0),
    reflectivity: rgb(0.5, 0.5, 0.5),
    roughness: 3,
  },
  {
    shape: "sphere",
    radius: MAIN_SIZE,
    position: vec3(0, 0, MAIN_Z),
    emission: rgb(0, 0, 0),
    reflectivity: rgb(0.5, 0.5, 0.5),
    roughness: 0,
  },
  {
    shape: "sphere",
    radius: MAIN_SIZE,
    position: vec3(MAIN_SIZE * 2.5, 0, MAIN_Z),
    emission: rgb(768, 0, 0),
    reflectivity: rgb(1.0, 0.0, 0.0),
    roughness: 0,
  },
  {
    shape: "sphere",
    radius: MAIN_SIZE,
    position: vec3(MAIN_SIZE * 2.5 * 0.6, 0, MAIN_Z + MAIN_SIZE * 2),
    emission: rgb(0, 0, 0),
    reflectivity: rgb(0.35, 1.0, 0.2),
    roughness: 0.1,
  },
  {
    shape: "sphere",
    radius: MAIN_SIZE,
    position: vec3(-(MAIN_SIZE * 2.5 * 0.6), 0, MAIN_Z + MAIN_SIZE * 2),
    emission: rgb(0, 0, 0),
    reflectivity: rgb(0.0, 0.0, 1.0),
    roughness: 0.1,
  },
];

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
          new WasmRGB(obj.reflectivity.r, obj.reflectivity.g, obj.reflectivity.b),
          obj.roughness,
          obj.radius,
        );
        scene.add_entity(entity);
      });

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
