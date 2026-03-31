/* eslint-disable no-restricted-globals */
import initWASM, { Scene, Entity, Vec3 as WasmVec3, Rgb as WasmRGB } from "wasm-lib";
import type { WorkerInMessage } from "./render.types";

console.log("renderer worker loaded");

const ctx = self as unknown as Worker;

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
    const position = new WasmVec3(obj.position.x, obj.position.y, obj.position.z);
    const emission = new WasmRGB(obj.emission.r, obj.emission.g, obj.emission.b);
    const albedo = new WasmRGB(obj.albedo.r, obj.albedo.g, obj.albedo.b);

    let entity: Entity;
    if (obj.shape === "plane") {
      entity = Entity.new_plane(
        position,
        new WasmVec3(obj.normal.x, obj.normal.y, obj.normal.z),
        emission,
        albedo,
        obj.metallic,
        obj.roughness,
      );
    } else {
      entity = Entity.new_sphere(position, emission, albedo, obj.metallic, obj.roughness, obj.radius);
    }
    scene.add_entity(entity);
  }

  scene.render(context as any);
};
