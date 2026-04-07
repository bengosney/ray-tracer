/* eslint-disable no-restricted-globals */
import initWASM, { Scene, Entity, Vec3 as WasmVec3, Rgb as WasmRGB, Material, Vec3, Rgb } from "wasm-lib";
import type { SceneObject, WorkerInMessage } from "./render.types";
import { exhaustiveMatchGuard } from "./utils/typeguard";
import { wasmRGB, wasmVec3 } from "./utils/conversions";

console.log("renderer worker loaded");

const CUBE_MODEL: string = `
v 0.0 0.0 0.0  # 1 a
v 0.0 1.0 0.0  # 2 b
v 1.0 1.0 0.0  # 3 c
v 1.0 0.0 0.0  # 4 d
v 0.0 0.0 1.0  # 5 e
v 0.0 1.0 1.0  # 6 f
v 1.0 1.0 1.0  # 7 g
v 1.0 0.0 1.0  # 8 h

vn  1.0  0.0  0.0  # 1 cghd
vn -1.0  0.0  0.0  # 2 aefb
vn  0.0  1.0  0.0  # 3 gcbf
vn  0.0 -1.0  0.0  # 4 dhea
vn  0.0  0.0  1.0  # 5 hgfe
vn  0.0  0.0 -1.0  # 6 cdab

f 3//1 7//1 8//1
f 3//1 8//1 4//1

f 1//2 5//2 6//2
f 1//2 6//2 2//2

f 7//3 3//3 2//3
f 7//3 2//3 6//3

f 4//4 8//4 5//4
f 4//4 5//4 1//4

f 8//5 7//5 6//5
f 8//5 6//5 5//5

f 3//6 4//6 1//6
f 3//6 1//6 2//6
`;

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
      return Entity.new_triangle(wasmVec3(obj.position), wasmVec3(obj.a), wasmVec3(obj.b), wasmVec3(obj.c), material);
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

  const model_mat = new Material(new Rgb(0, 0, 0), new Rgb(1, 0, 0), 0, 0, 0, 1.5);
  scene.load_model(CUBE_MODEL, new WasmVec3(0, 0, 5), model_mat);

  scene.set_gamma_correction(gamma);

  // for (const obj of entities) {
  //   const entity: Entity = createEntity(obj);
  //   scene.add_entity(entity);
  // }

  scene.render(context as any);
};
