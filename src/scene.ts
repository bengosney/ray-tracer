import { vec3, mag, sub, createRng } from "./utils/math";
import { rgb } from "./utils/colour";
import type { SceneObject, ModelData } from "./render.types";
import RABBIT_MODEL from "./models/rabbit";

const MAIN_SIZE = 25;

const BASE_SCENE: SceneObject[] = [
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
    position: vec3(0, 0, 150),
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
    position: vec3(MAIN_SIZE * 2.5, 0, 150),
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
    position: vec3(MAIN_SIZE * 2.5 * 0.6, 0, 150 + MAIN_SIZE * 2),
    emission: rgb(0, 0, 0),
    albedo: rgb(0.6, 0.92, 0.2),
    metallic: 1.0,
    roughness: 0.1,
    transmission: 0.0,
    ior: 1.5,
  },
  // forward
  {
    shape: "sphere",
    radius: MAIN_SIZE,
    position: vec3(-(MAIN_SIZE * 2.5 * 0.6), 0, 150 - MAIN_SIZE * 1),
    emission: rgb(0, 0, 0),
    albedo: rgb(0.1, 0.3, 1.0),
    metallic: 0.0,
    roughness: 0.2,
    transmission: 0.0,
    ior: 1.5,
  },
];

export const RABBIT_MODEL_DATA: ModelData = {
  obj: RABBIT_MODEL,
  position: vec3(15, 34, 100),
  rotation: vec3(Math.PI, 0, 0),
  scale: 250,
  emission: rgb(0, 0, 0),
  albedo: rgb(0.76, 0.46, 0.33),
  metallic: 0,
  roughness: 0.8,
  transmission: 0.0,
  ior: 1.5,
};

export function buildSceneData(sphereCount: number, focalDistance: number, seed: number): SceneObject[] {
  const random = createRng(seed);
  const scene: SceneObject[] = [...BASE_SCENE];

  for (let i = 0; i < sphereCount; i++) {
    for (let attempts = 0; attempts < 100; attempts++) {
      const radius = 2 + random() * 5;
      const x = (random() - 0.5) * 150;
      const z = focalDistance + (random() - 0.5) * 175;
      const position = vec3(x, MAIN_SIZE - radius, z);

      let isIntersecting = false;
      for (const obj of scene) {
        if (obj.shape === "sphere") {
          if (mag(sub(position, obj.position)) < radius + obj.radius) {
            isIntersecting = true;
            break;
          }
        }
      }

      if (!isIntersecting) {
        const metallic = random() > 0.6 ? 1.0 : 0.0;
        const transmission = !metallic && random() > 0.7 ? 1.0 : 0.0;

        scene.push({
          shape: "sphere",
          radius,
          position,
          emission: rgb(0, 0, 0),
          albedo: rgb(random(), random(), random()),
          metallic,
          roughness: random(),
          transmission,
          ior: 1.5,
        });

        break;
      }
    }
  }

  return scene;
}
