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

export interface SceneEntity {
  shape: "sphere" | "plane";
  position: { x: number; y: number; z: number };
  emission: { r: number; g: number; b: number };
  albedo: { r: number; g: number; b: number };
  metallic: number;
  roughness: number;
  radius?: number;
  normal?: { x: number; y: number; z: number };
}

export type WorkerInMessage = {
  type: "start";
  canvas: OffscreenCanvas;
  settings: WorkerSettings;
  entities: SceneEntity[];
  gamma: number;
};

export type WorkerOutMessage = { type: "done" };
