export interface Vec2 {
  x: number;
  y: number;
}

export interface Vec3 {
  x: number;
  y: number;
  z: number;
}

export const degreeToRadians = (degree: number): number => (degree * Math.PI) / 180;

export const vec2 = (x: number, y: number): Vec2 => ({ x, y });

export const vec3 = (x: number, y: number, z: number): Vec3 => ({ x, y, z });

export const add = (a: Vec3, b: Vec3): Vec3 => vec3(a.x + b.x, a.y + b.y, a.z + b.z);

export const sub = (a: Vec3, b: Vec3): Vec3 => vec3(a.x - b.x, a.y - b.y, a.z - b.z);

export const mag = ({ x, y, z }: Vec3): number => Math.sqrt(x ** 2 + y ** 2 + z ** 2);

export const mul = ({ x, y, z }: Vec3, s: number): Vec3 => vec3(x * s, y * s, z * s);

export const avg = (v: Vec3[]): Vec3 =>
  mul(
    v.reduce((acc, cur) => add(acc, cur), vec3(0, 0, 0)),
    1 / v.length,
  );

export const mulParts = (a: Vec3, b: Vec3): Vec3 => vec3(a.x * b.x, a.y * b.y, a.z * b.z);

export const dot = (a: Vec3, b: Vec3): number => a.x * b.x + a.y * b.y + a.z * b.z;

export const normalize = (vec: Vec3): Vec3 => mul(vec, 1 / mag(vec));

export const reflect = (direction: Vec3, normal: Vec3) => sub(direction, mul(normal, dot(direction, normal) * 2));
