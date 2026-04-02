import type { WorkerSettings } from "./render.types";

export interface Settings extends WorkerSettings {
  gamma: number;
}

export type SettingsAction = { type: "set"; settings: Partial<Settings> } | { type: "reset" };

const FOCAL_LENGTH = 1000;
const FOCAL_DISTANCE = FOCAL_LENGTH / 4;
const APERTURE = FOCAL_DISTANCE / 200;

export const DEFAULT_SETTINGS: Settings = {
  width: 640,
  height: 480,
  focalLength: FOCAL_LENGTH,
  focalDistance: FOCAL_DISTANCE,
  aperture: APERTURE,
  samples: 500,
  bounces: 50,
  fov: 80,
  gamma: 2.2,
};

export function settingsReducer(state: Settings, action: SettingsAction): Settings {
  switch (action.type) {
    case "set": {
      return { ...state, ...action.settings };
    }
    case "reset":
      return DEFAULT_SETTINGS;
  }
}
