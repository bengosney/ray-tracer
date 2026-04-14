import { useState, useCallback } from "react";
import type { CameraSettings } from "./RenderSettings";

interface CameraControlsProps {
  camera: CameraSettings;
  onCameraChange: (camera: CameraSettings) => void;
}

function CameraControls({ camera, onCameraChange }: CameraControlsProps) {
  const [speed, setSpeed] = useState(5);

  const move = useCallback(
    (dx: number, dy: number, dz: number) => {
      onCameraChange({
        ...camera,
        position: {
          x: camera.position.x + dx * speed,
          y: camera.position.y + dy * speed,
          z: camera.position.z + dz * speed,
        },
      });
    },
    [camera, speed, onCameraChange],
  );

  const rotate = useCallback(
    (dx: number, dy: number, dz: number) => {
      const step = 0.01 * speed;
      onCameraChange({
        ...camera,
        rotation: {
          x: camera.rotation.x + dx * step,
          y: camera.rotation.y + dy * step,
          z: camera.rotation.z + dz * step,
        },
      });
    },
    [camera, speed, onCameraChange],
  );

  return (
    <fieldset>
      <legend>Camera Controls</legend>
      <ul>
        <li>
          <label>
            Speed
            <input type="range" min={1} max={50} value={speed} onChange={(e) => setSpeed(e.target.valueAsNumber)} />
          </label>
        </li>
      </ul>
      <div className="camera-controls">
        <div className="control-group">
          <span className="control-label">Move</span>
          <div className="control-grid">
            <button onClick={() => move(0, -1, 0)}>Up</button>
            <button onClick={() => move(0, 0, 1)}>Fwd</button>
            <button onClick={() => move(0, 1, 0)}>Down</button>
            <button onClick={() => move(-1, 0, 0)}>Left</button>
            <button onClick={() => move(0, 0, -1)}>Back</button>
            <button onClick={() => move(1, 0, 0)}>Right</button>
          </div>
        </div>
        <div className="control-group">
          <span className="control-label">Rotate</span>
          <div className="control-grid">
            <button onClick={() => rotate(0, 0, -1)}>CCW</button>
            <button onClick={() => rotate(1, 0, 0)}>Up</button>
            <button onClick={() => rotate(0, 0, 1)}>CW</button>
            <button onClick={() => rotate(0, -1, 0)}>Left</button>
            <button onClick={() => rotate(-1, 0, 0)}>Down</button>
            <button onClick={() => rotate(0, 1, 0)}>Right</button>
          </div>
        </div>
      </div>
    </fieldset>
  );
}

export default CameraControls;
