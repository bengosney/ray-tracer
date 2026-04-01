/* eslint-disable no-restricted-globals */
import initWASM, { Scene, Entity, Vec3 as WasmVec3, Rgb as WasmRGB, Material } from "wasm-lib";
import type { SceneObject, WorkerInMessage } from "./render.types";
import { exhaustiveMatchGuard } from "./utils/typeguard";

console.log("renderer worker loaded");

const ctx = self as unknown as Worker;

const createEntity = (obj: SceneObject): Entity => {
  const position = new WasmVec3(obj.position.x, obj.position.y, obj.position.z);

  const material = new Material(
    new WasmRGB(obj.emission.r, obj.emission.g, obj.emission.b),
    new WasmRGB(obj.albedo.r, obj.albedo.g, obj.albedo.b),
    obj.metallic,
    obj.roughness,
    obj.transmission,
    obj.ior,
  );

  const shape = obj.shape;
  switch (shape) {
    case "plane":
      const normal = new WasmVec3(obj.normal.x, obj.normal.y, obj.normal.z);
      return Entity.new_plane(position, material, normal);
    case "sphere":
      return Entity.new_sphere(position, material, obj.radius);
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
