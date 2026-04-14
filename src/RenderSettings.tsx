import { useState, useEffect, useRef, useCallback } from "react";

export interface Settings {
  render: {
    width: number;
    height: number;
    focalLength: number;
    focalDistance: number;
    aperture: number;
    samples: number;
    bounces: number;
    gamma: number;
  };
  scene: {
    seed: number;
    showRabbit: boolean;
    sphereCount: number;
  };
}

interface RenderSettingsProps {
  settings: Settings;
  onSettingsChange: (settings: Settings) => void;
}

function RenderSettings({ settings, onSettingsChange }: RenderSettingsProps) {
  const [local, setLocal] = useState(settings);
  const timerRef = useRef<ReturnType<typeof setTimeout>>();

  useEffect(() => setLocal(settings), [settings]);

  const debouncedChange = useCallback(
    (next: Settings) => {
      clearTimeout(timerRef.current);
      timerRef.current = setTimeout(() => onSettingsChange(next), 300);
    },
    [onSettingsChange],
  );

  useEffect(() => () => clearTimeout(timerRef.current), []);

  const updateRender = (patch: Partial<Settings["render"]>) => {
    const next = { ...local, render: { ...local.render, ...patch } };
    setLocal(next);
    debouncedChange(next);
  };

  const updateScene = (patch: Partial<Settings["scene"]>) => {
    const next = { ...local, scene: { ...local.scene, ...patch } };
    setLocal(next);
    debouncedChange(next);
  };

  return (
    <div className="settings">
      <fieldset>
        <legend>Render</legend>
        <ul>
          <li>
            <label>
              Samples
              <input
                type="number"
                min={1}
                value={local.render.samples}
                onChange={(e) => updateRender({ samples: e.target.valueAsNumber })}
              />
            </label>
          </li>
          <li>
            <label>
              Bounces
              <input
                type="number"
                min={1}
                value={local.render.bounces}
                onChange={(e) => updateRender({ bounces: e.target.valueAsNumber })}
              />
            </label>
          </li>
          <li>
            <label>
              Focal Length
              <input
                type="number"
                min={1}
                value={local.render.focalLength}
                onChange={(e) => updateRender({ focalLength: e.target.valueAsNumber })}
              />
            </label>
          </li>
          <li>
            <label>
              Focal Distance
              <input
                type="number"
                min={1}
                value={local.render.focalDistance}
                onChange={(e) => updateRender({ focalDistance: e.target.valueAsNumber })}
              />
            </label>
          </li>
          <li>
            <label>
              Aperture
              <input
                type="number"
                min={0}
                step={0.01}
                value={local.render.aperture}
                onChange={(e) => updateRender({ aperture: e.target.valueAsNumber })}
              />
            </label>
          </li>
          <li>
            <label>
              Gamma
              <input
                type="number"
                min={1}
                step={0.1}
                value={local.render.gamma}
                onChange={(e) => updateRender({ gamma: e.target.valueAsNumber })}
              />
            </label>
          </li>
        </ul>
      </fieldset>

      <fieldset>
        <legend>Scene</legend>
        <ul>
          <li>
            <label>
              Seed
              <input
                type="number"
                min={0}
                max={9_999_999}
                value={local.scene.seed}
                onChange={(e) => updateScene({ seed: e.target.valueAsNumber })}
              />
            </label>
          </li>
          <li>
            <label>
              Sphere Count
              <input
                type="number"
                min={0}
                value={local.scene.sphereCount}
                onChange={(e) => updateScene({ sphereCount: e.target.valueAsNumber })}
              />
            </label>
          </li>
          <li>
            <label>
              Rabbit
              <input
                type="checkbox"
                checked={local.scene.showRabbit}
                onChange={(e) => updateScene({ showRabbit: e.target.checked })}
              />
            </label>
          </li>
        </ul>
      </fieldset>
    </div>
  );
}

export default RenderSettings;
