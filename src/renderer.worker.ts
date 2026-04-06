/* eslint-disable no-restricted-globals */
import initWASM, { Scene, Entity, Vec3 as WasmVec3, Rgb as WasmRGB, Material } from "wasm-lib";
import type { SceneObject, WorkerInMessage } from "./render.types";
import { exhaustiveMatchGuard } from "./utils/typeguard";
import { wasmRGB, wasmVec3 } from "./utils/conversions";

console.log("renderer worker loaded");

const ctx = self as unknown as Worker;

const createEntity = (obj: SceneObject): Entity => {
  const material = new Material(
    wasmRGB(obj.emission),
    wasmRGB(obj.albedo),
    obj.metallic,
    obj.roughness,
    obj.transmission,
    obj.ior,
  );

  const shape = obj.shape;
  switch (shape) {
    case "plane":
      return Entity.new_plane(wasmVec3(obj.position), material, wasmVec3(obj.normal));
    case "sphere":
      return Entity.new_sphere(wasmVec3(obj.position), material, obj.radius);
    case "triangle":
      return Entity.new_triangle(wasmVec3(obj.a), wasmVec3(obj.b), wasmVec3(obj.c), material);
    default:
      return exhaustiveMatchGuard(`unknow shapre: ${shape}`);
  }
};

ctx.onmessage = async (e: MessageEvent<WorkerInMessage>) => {
  console.log("renderer worker message", e.data);
  if (e.data.type !== "start") return;

  const { canvas, settings, entities, gamma } = e.data;

  await initWASM();

  const context = canvas.getContext("2d")!;

  const scene = new Scene(
    settings.width,
    settings.height,
    settings.focalLength,
    settings.focalDistance,
    settings.aperture,
    settings.samples,
    settings.bounces,
    settings.fov,
  );

  scene.set_gamma_correction(gamma);

  for (const obj of entities) {
    const entity: Entity = createEntity(obj);
    scene.add_entity(entity);
  }

  scene.render(context as any);
};
