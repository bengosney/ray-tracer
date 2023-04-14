import { useCallback, useEffect, useRef, useState } from "react";
import "./App.css";
import Canvas from "./Canvas";
import useMaxSize, { ASPECT_4_3 } from "./hooks/useMaxSize";
import { Vec2, Vec3, add, vec3, normalize, mul, sub, mag, dot, reflect, mulParts, avg } from "./utils/math";
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

type Sceen = Array<Objects>;

const objects: Sceen = [
  {
    shape: "sphere",
    position: vec3(1000, 0, 0),
    radius: 990,
    emission: rgb(0, 0, 0),
    reflectivity: rgb(0.8, 0, 0),
    roughness: 10,
  },
  {
    shape: "sphere",
    position: vec3(-1000, 0, 0),
    radius: 990,
    emission: rgb(0, 0, 0),
    reflectivity: rgb(0, 0.8, 0),
    roughness: 3,
  },
  {
    shape: "sphere",
    position: vec3(0, 1000, 0),
    radius: 990,
    emission: rgb(0, 0, 0),
    reflectivity: rgb(0.8, 0.8, 0.8),
    roughness: 3,
  },
  {
    shape: "sphere",
    position: vec3(0, -1000, 0),
    radius: 990,
    emission: rgb(0, 0, 0),
    reflectivity: rgb(0.8, 0.8, 0.8),
    roughness: 3,
  },
  {
    shape: "sphere",
    position: vec3(0, 0, 1000),
    radius: 990,
    emission: rgb(0, 0, 0),
    reflectivity: rgb(0.8, 0.8, 0.8),
    roughness: 3,
  },
  {
    shape: "sphere",
    position: vec3(0, -14.5, 7),
    radius: 5,
    emission: rgb(5550, 5550, 5550),
    reflectivity: rgb(0.8, 0.8, 0.8),
    roughness: 3,
  },
  {
    shape: "sphere",
    position: vec3(3, 7, 7),
    radius: 3,
    emission: rgb(0, 0, 0),
    reflectivity: rgb(0.8, 0.8, 0.8),
    roughness: 0,
  },
];

interface IntersectionResult {
  collided: boolean;
  dist: number;
  point: Vec3;
  normal: Vec3;
  object?: Objects;
}

const sphereIntersection = (origin: Vec3, direction: Vec3, sphere: Sphere): IntersectionResult => {
  const sphereRay = sub(sphere.position, origin);
  const distSphereRay = mag(sphereRay);
  const distToClosestPointOnRay = dot(sphereRay, direction);
  const distFromClosestPointToSphere = Math.sqrt(distSphereRay ** 2 - distToClosestPointOnRay ** 2);

  const distToIntersection =
    distToClosestPointOnRay - Math.sqrt(Math.abs(sphere.radius ** 2 - distFromClosestPointToSphere ** 2));
  const point = add(origin, mul(direction, distToIntersection));
  let normal = normalize(sub(point, sphere.position));

  normal = normalize(
    add(normal, mul(vec3(Math.random() - 0.5, Math.random() - 0.5, Math.random() - 0.5), sphere.roughness)),
  );

  if (distToClosestPointOnRay > 0 && distFromClosestPointToSphere < sphere.radius) {
    return {
      collided: true,
      dist: distToIntersection,
      point: point,
      normal: normal,
    };
  }

  return {
    collided: false,
    dist: Infinity,
    point: vec3(0, 0, 0),
    normal: vec3(0, 0, 0),
  };
};

const intersection = (origin: Vec3, direction: Vec3, sceen: Sceen): IntersectionResult => {
  const closestIntersection: IntersectionResult = {
    collided: false,
    point: vec3(0, 0, 0),
    dist: Infinity,
    normal: vec3(0, 0, 0),
    object: undefined,
  };

  sceen.forEach((object) => {
    switch (object.shape) {
      case "sphere":
        const intersection = sphereIntersection(origin, direction, object);

        if (intersection.dist < closestIntersection.dist) {
          closestIntersection.dist = intersection.dist;
          closestIntersection.object = object;
          closestIntersection.normal = intersection.normal;
          closestIntersection.point = intersection.point;
        }

        closestIntersection.collided = closestIntersection.collided || intersection.collided;
        break;
    }
  });

  return closestIntersection;
};

const trace = (origin: Vec3, direction: Vec3, sceen: Sceen, steps: number): Vec3 => {
  const intersect = intersection(origin, direction, sceen);

  if (intersect.collided && steps > 0 && intersect.object !== undefined) {
    const reflectedDirection = reflect(direction, intersect.normal);

    const bounce = trace(
      intersect.point,
      reflectedDirection,
      objects.filter((o) => o != intersect.object),
      steps - 1,
    );

    return add(rgbToVec3(intersect.object?.emission), mulParts(bounce, rgbToVec3(intersect.object.reflectivity)));
  }

  return vec3(0, 0, 0);
};

const render = (
  width: number,
  height: number,
  focalLength: number,
  sampleCount: number,
  bounces: number,
  drawPixel: any,
  [start, finish]: [any, any],
) => {
  const halfWidth = Math.floor(width / 2);
  const halfHeight = Math.floor(height / 2);
  const origin = vec3(0, 0, 0);
  const samples: Vec3[][][] = [];
  const timeouts: ReturnType<typeof setTimeout>[] = [];

  for (let s = 0; s < sampleCount; s++) {
    start();
    const timeout = setTimeout(() => {
      for (let i = 0; i < width; i++) {
        for (let j = 0; j < height; j++) {
          const x = i - halfWidth;
          const y = j - halfHeight;
          const direction = normalize(vec3(x, y, focalLength));

          samples[x] = samples[x] || [];
          samples[x][y] = samples[x][y] || [];

          samples[x][y].push(trace(origin, direction, objects, bounces));
          const colour = avg(samples[x][y]);

          drawPixel({ x: i, y: j }, vec3ToRGB(colour));
        }
      }
      finish();
    }, 1);
    timeouts.push(timeout);
  }

  return timeouts;
};

function App() {
  //const { width, height } = useMaxSize(ASPECT_4_3);
  const width = 320;
  const height = 240;
  const focalLength = 50;
  const samples = 3;
  const bounces = 4;
  const imageData = useRef<ImageData>();
  const [ready, setReady] = useState<boolean>(false);
  const [samplesStarted, setSamplesStarted] = useState<number>(0);
  const addSampleStarted = () => setSamplesStarted((s) => s + 1);
  const [samplesFinished, setSamplesFinished] = useState<number>(0);
  const addSampleFinished = () => setSamplesFinished((s) => s + 1);

  useEffect(() => {
    if (ready) {
      initWASM().then(() => {
        const scene = new Scene(320, 240, 50, 1, 4);
        objects.forEach((o) => {
          const entity = new Entity(
            wasmShape.Sphere,
            new wasmVec3(o.position.x, o.position.y, o.position.z),
            new wasmRGB(o.emission.r, o.emission.g, o.emission.b),
            new wasmRGB(o.reflectivity.r, o.reflectivity.g, o.reflectivity.b),
            o.roughness,
            o.radius,
          );
          scene.add_entity(entity);
        });
        console.log("stating render");
        const pixels = scene.render();
        console.log("scene", pixels);
        if (imageData.current !== undefined) {
          for (let i = 0; i < pixels.length; i++) {
            imageData.current.data[i] = pixels[i];
          }
        }
      });
    }
  }, [ready]);

  const drawPixel = useCallback(
    ({ x, y }: Vec2, colour: RGB) => {
      const offset = 4 * (Math.floor(x) + Math.floor(y) * width);
      if (imageData.current !== undefined) {
        imageData.current.data[offset] = colour.r;
        imageData.current.data[offset + 1] = colour.g;
        imageData.current.data[offset + 2] = colour.b;
        imageData.current.data[offset + 3] = 255;
      }
    },
    [width],
  );

  const init = useCallback((context: CanvasRenderingContext2D) => {
    const width = Math.floor(context.canvas.width);
    const height = Math.floor(context.canvas.height);
    imageData.current = context.createImageData(width, height);
    setReady(true);
  }, []);

  useEffect(() => {
    if (imageData.current && false) {
      const timeouts = render(width, height, focalLength, samples, bounces, drawPixel, [
        addSampleStarted,
        addSampleFinished,
      ]);

      return () => timeouts.forEach((t) => clearTimeout(t));
    }
  }, [ready, imageData.current]);

  const frame = useCallback((context: CanvasRenderingContext2D, since: number) => {
    if (imageData.current !== undefined) {
      context.putImageData(imageData.current, 0, 0);
    }
  }, []);

  return (
    <div className="App">
      {samplesStarted > samplesFinished ? <div>Rendering</div> : null}
      <Canvas animating={samplesStarted > samplesFinished} width={width} height={height} init={init} frame={frame} />
    </div>
  );
}

export default App;
