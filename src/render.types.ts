import type { Vec3 } from "./utils/math";
import type { RGB } from "./utils/colour";

interface BaseObject {
  position: Vec3;
  emission: RGB;
  albedo: RGB;
  metallic: number;
  roughness: number;
}

export interface Sphere extends BaseObject {
  shape: "sphere";
  radius: number;
}

export interface Plane extends BaseObject {
  shape: "plane";
  normal: Vec3;
}

export type SceneObject = Sphere | Plane;

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
