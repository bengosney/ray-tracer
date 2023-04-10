import { useCallback, useRef } from "react";
import "./App.css";
import Canvas from "./Canvas";
import useMaxSize, { ASPECT_4_3 } from "./hooks/useMaxSize";
import { Vec2, Vec3, add, vec2, vec3, normalize, mul } from "./utils/math";
import { RGB, rgb, vec3ToRGB } from "./utils/colour";

type Shape = "sphere";

interface Object {
  shape: Shape;
  position: Vec3;
  colour: RGB;
}

interface Sphere extends Object {
  shape: "sphere";
  radius: number;
}

type Objects = Sphere;

const objects: Objects[] = [
  {
    shape: "sphere",
    radius: 3,
    position: vec3(3, 7, 7),
    colour: rgb(255, 0, 0),
  },
];

function App() {
  const { width, height } = useMaxSize(ASPECT_4_3);
  const focalLength = 50;
  const samples = 100;
  const imageData = useRef<ImageData>();

  const drawPixel = useCallback(
    ({ x, y }: Vec2, color: RGB) => {
      const offset = 4 * (Math.floor(x) + Math.floor(y) * width);
      if (imageData.current !== undefined) {
        imageData.current.data[offset] = color.r;
        imageData.current.data[offset + 1] = color.g;
        imageData.current.data[offset + 2] = color.b;
        imageData.current.data[offset + 3] = 255;
      }
    },
    [width],
  );

  const init = useCallback((context: CanvasRenderingContext2D) => {
    const width = Math.floor(context.canvas.width);
    const height = Math.floor(context.canvas.height);
    imageData.current = context.createImageData(width, height);
  }, []);

  const trace = (origin: Vec3, direction: Vec3, objects: Objects[], steps: number): Vec3 => {
    return vec3(0, 0, 0);
  };

  const frame = useCallback((context: CanvasRenderingContext2D, since: number) => {
    for (let x = 0; x < width; x++) {
      for (let y = 0; y < height; y++) {
        const direction = normalize(vec3(x, y, focalLength));

        let color = vec3(0, 0, 0);
        for (let i = 0; i < samples; i++) {
          color = add(color, trace(vec3(0, 0, 0), direction, objects, 4));
        }
        color = mul(color, 1 / samples);

        drawPixel({ x, y }, vec3ToRGB(color));
      }
    }

    if (imageData.current !== undefined) {
      context.putImageData(imageData.current, 0, 0);
    }
  }, []);

  return (
    <div className="App">
      <Canvas animating={false} width={width} height={height} init={init} frame={frame} />
    </div>
  );
}

export default App;
