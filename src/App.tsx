import { useCallback, useEffect, useRef, useState } from "react";
import "./App.css";
import Canvas from "./Canvas";
import useMaxSize, { ASPECT_4_3 } from "./hooks/useMaxSize";
import { Vec3, vec3 } from "./utils/math";
import { RGB, rgb, rgbToVec3, vec3ToRGB } from "./utils/colour";

import initWASM, { Scene, Entity, Shape as wasmShape, Vec3 as wasmVec3, RGB as wasmRGB } from "wasm-lib";

type Shape = "sphere" | "cube";

interface Object {
  shape: Shape;
  position: Vec3;
  emission: RGB;
  reflectivity: RGB;
  roughness: number;
}

interface Sphere extends Object {
  shape: "sphere";
  radius: number;
}

interface Cube extends Object {
  shape: "cube";
}

type Objects = Sphere; // | Cube;

type SceneObjects = Array<Objects>;

const objects: SceneObjects = [
  {
    shape: "sphere",
    position: vec3(1000, 0, 0),
    radius: 990,
    emission: rgb(0, 0, 0),
    reflectivity: rgb(0.5, 0, 0),
    roughness: 10,
  },
  {
    shape: "sphere",
    position: vec3(-1000, 0, 0),
    radius: 990,
    emission: rgb(0, 0, 0),
    reflectivity: rgb(0, 0.5, 0),
    roughness: 3,
  },
  {
    shape: "sphere",
    position: vec3(0, 1000, 0),
    radius: 990,
    emission: rgb(0, 0, 0),
    reflectivity: rgb(0.5, 0.5, 0.5),
    roughness: 3,
  },
  {
    shape: "sphere",
    position: vec3(0, -1000, 0),
    radius: 990,
    emission: rgb(0, 0, 0),
    reflectivity: rgb(0.5, 0.5, 0.5),
    roughness: 3,
  },
  {
    shape: "sphere",
    position: vec3(0, 0, 1000),
    radius: 990,
    emission: rgb(0, 0, 0),
    reflectivity: rgb(0.5, 0.5, 0.5),
    roughness: 3,
  },
  {
    shape: "sphere",
    position: vec3(0, -14.5, 7),
    radius: 5,
    emission: rgb(5550, 5550, 5550),
    reflectivity: rgb(0.5, 0.5, 0.5),
    roughness: 0,
  },
  {
    shape: "sphere",
    position: vec3(3, 7, 7),
    radius: 3,
    emission: rgb(0, 0, 0),
    reflectivity: rgb(1, 1, 1),
    roughness: 0,
  },
];

function App() {
  //const { width, height } = useMaxSize(ASPECT_4_3);
  const width = 320 * 2;
  const height = 240 * 2;
  const focalLength = 100;
  const samples = 10;
  const bounces = 4;
  const [context, setContext] = useState<CanvasRenderingContext2D>();

  const mat = rgb(150, 150, 150);
  const sceneObjects: SceneObjects = [
    {
      shape: "sphere",
      radius: 8,
      position: vec3(0, 0, 10),
      emission: mat,
      reflectivity: rgb(0.5, 0.5, 0.5),
      roughness: 0,
    },
    {
      shape: "sphere",
      position: vec3(0, 1000, 0),
      radius: 990,
      emission: mat,
      reflectivity: rgb(0.5, 0.5, 0.5),
      roughness: 3,
    },
  ];

  useEffect(() => {
    if (context) {
      initWASM().then(() => {
        const scene = new Scene(
          context.canvas.width,
          context.canvas.height,
          focalLength,
          samples,
          bounces,
          new wasmRGB(0, 0, 0),
          //new wasmRGB(253, 244, 220),
          //new wasmRGB(100, 100, 100),
        );
        sceneObjects.forEach((o) => {
          const entity = new Entity(
            new wasmVec3(o.position.x, o.position.y, o.position.z),
            new wasmRGB(o.emission.r, o.emission.g, o.emission.b),
            new wasmRGB(o.reflectivity.r, o.reflectivity.g, o.reflectivity.b),
            o.roughness,
            o.radius,
          );
          scene.add_entity(entity);
        });

        scene.render(context);
      });
    }
  }, [context]);

  const init = useCallback((context: CanvasRenderingContext2D) => {
    setContext(context);
  }, []);

  return (
    <div className="App">
      <Canvas animating={false} width={width} height={height} init={init} frame={() => {}} />
    </div>
  );
}

export default App;
