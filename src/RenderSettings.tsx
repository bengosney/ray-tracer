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
    showRabbit: boolean;
    sphereCount: number;
  };
}

interface RenderSettingsProps {
  settings: Settings;
  onSettingsChange: (settings: Settings) => void;
}

function RenderSettings({ settings, onSettingsChange }: RenderSettingsProps) {
  const updateRender = (patch: Partial<Settings["render"]>) =>
    onSettingsChange({ ...settings, render: { ...settings.render, ...patch } });

  const updateScene = (patch: Partial<Settings["scene"]>) =>
    onSettingsChange({ ...settings, scene: { ...settings.scene, ...patch } });

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
                value={settings.render.samples}
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
                value={settings.render.bounces}
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
                value={settings.render.focalLength}
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
                value={settings.render.focalDistance}
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
                value={settings.render.aperture}
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
                value={settings.render.gamma}
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
              Sphere Count
              <input
                type="number"
                min={0}
                value={settings.scene.sphereCount}
                onChange={(e) => updateScene({ sphereCount: e.target.valueAsNumber })}
              />
            </label>
          </li>
          <li>
            <label>
              Rabbit
              <input
                type="checkbox"
                checked={settings.scene.showRabbit}
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
