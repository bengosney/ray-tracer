import { useCallback, useRef } from "react";
import "./App.css";
import Canvas from "./Canvas";
import useMaxSize, { ASPECT_4_3 } from "./hooks/useMaxSize";
import { Vec2, Vec3, vec2 } from "./utils/math";
import { RGB, rgb } from "./utils/colour";

function App() {
  const { width, height } = useMaxSize(ASPECT_4_3);
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

  const frame = useCallback((context: CanvasRenderingContext2D, since: number) => {
    for (let i = 0; i < Math.min(width, height); i++) {
      drawPixel(vec2(i, i), rgb(255, 0, 0));
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
