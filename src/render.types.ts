import type { Vec3 } from "./utils/math";
import type { RGB } from "./utils/colour";

interface BaseObject {
  emission: RGB;
  albedo: RGB;
  metallic: number;
  roughness: number;
  transmission: number;
  ior: number;
}

export interface Sphere extends BaseObject {
  shape: "sphere";
  radius: number;
  position: Vec3;
}

export interface Plane extends BaseObject {
  shape: "plane";
  normal: Vec3;
  position: Vec3;
}

export interface Triangle extends BaseObject {
  shape: "triangle";
  a: Vec3;
  b: Vec3;
  c: Vec3;
  position: Vec3;
}

export type SceneObject = Sphere | Plane | Triangle;

export interface WorkerSettings {
  width: number;
  height: number;
  focalLength: number;
  focalDistance: number;
  aperture: number;
  samples: number;
  bounces: number;
  fov: number;
}

export type WorkerInMessage = {
  type: "start";
  canvas: OffscreenCanvas;
  settings: WorkerSettings;
  entities: SceneObject[];
  gamma: number;
};

export type WorkerOutMessage = { type: "done" };
