import { useState, useCallback } from "react";
import "./App.css";
import CameraControls from "./CameraControls";
import RenderSettings, { type Settings } from "./RenderSettings";
import RenderStats from "./RenderStats";
import { useRenderer } from "./hooks/useRenderer";

const DEFAULT_SETTINGS: Settings = {
  render: {
    width: 640,
    height: 480,
    samples: 500,
    bounces: 50,
    gamma: 2.2,
  },
  camera: {
    position: { x: 0, y: 0, z: 0 },
    rotation: { x: 0, y: 0, z: 0 },
    focalLength: 550,
    focalDistance: 150,
    aperture: 0.1,
  },
  scene: {
    seed: Math.floor(Math.random() * 9_999_999),
    showRabbit: false,
    sphereCount: 250,
  },
};

function App() {
  const [settings, setSettings] = useState<Settings>(DEFAULT_SETTINGS);
  const { rendering, stats, handlers } = useRenderer(settings);

  const handleSettingsChange = useCallback((next: Settings) => {
    setSettings(next);
  }, []);

  return (
    <div className="App">
      <canvas
        key={`${JSON.stringify(settings)}-${rendering.renderKey}`}
        ref={rendering.canvasRefCallback}
        width={settings.render.width}
        height={settings.render.height}
      />
      {stats.renderStats ? (
        <RenderStats
          sampleIndex={stats.renderStats.sampleIndex}
          totalSamples={settings.render.samples}
          lastDurationMs={stats.renderStats.durationMs}
          sampleTimes={stats.sampleTimes}
        />
      ) : (
        <p className="stats">Initialising Render...</p>
      )}
      <div className="settings">
        <CameraControls
          camera={settings.camera}
          onCameraChange={(camera) => handleSettingsChange({ ...settings, camera })}
        />
      </div>
      <RenderSettings settings={settings} onSettingsChange={handleSettingsChange} />
      <fieldset className="settings">
        <legend>Controls</legend>
        <div className="controls">
          <button onClick={handlers.handleSave}>Save PNG</button>
          <button onClick={handlers.handleStop} disabled={!stats.running}>
            Stop
          </button>
          <button onClick={handlers.handleRestart}>Restart</button>
        </div>
      </fieldset>
    </div>
  );
}

export default App;
