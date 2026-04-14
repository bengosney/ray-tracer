import { useRef, useState, useCallback } from "react";
import type { WorkerInMessage, WorkerOutMessage } from "../render.types";
import type { Settings } from "../RenderSettings";
import { buildSceneData, RABBIT_MODEL_DATA } from "../scene";

interface RenderStats {
  sampleIndex: number;
  durationMs: number;
}

interface UseRendererReturn {
  rendering: {
    canvasRefCallback: (canvas: HTMLCanvasElement | null) => void;
    renderKey: number;
  };
  stats: {
    running: boolean;
    renderStats: RenderStats | null;
    sampleTimes: number[];
  };
  handlers: {
    handleStop: () => void;
    handleRestart: () => void;
    handleSave: () => void;
  };
}

function startRender(canvas: HTMLCanvasElement, settings: Settings): Worker {
  const worker = new Worker(new URL("../renderer.worker.ts", import.meta.url));
  const offscreen = canvas.transferControlToOffscreen();
  worker.onerror = (e) => console.error("worker error:", e);

  const msg: WorkerInMessage = {
    type: "start",
    canvas: offscreen,
    settings: {
      ...settings.render,
      cameraPosition: settings.camera.position,
      cameraRotation: settings.camera.rotation,
      focalLength: settings.camera.focalLength,
      focalDistance: settings.camera.focalDistance,
      aperture: settings.camera.aperture,
    },
    entities: buildSceneData(settings.scene.sphereCount, settings.camera.focalDistance, settings.scene.seed),
    models: settings.scene.showRabbit ? [RABBIT_MODEL_DATA] : [],
    gamma: settings.render.gamma,
  };
  worker.postMessage(msg, [offscreen]);

  return worker;
}

export const useRenderer = (settings: Settings): UseRendererReturn => {
  const workerRef = useRef<Worker | null>(null);
  const canvasRef = useRef<HTMLCanvasElement | null>(null);
  const [renderKey, setRenderKey] = useState(0);
  const [running, setRunning] = useState(false);
  const [renderStats, setRenderStats] = useState<RenderStats | null>(null);
  const [sampleTimes, setSampleTimes] = useState<number[]>([]);

  const canvasRefCallback = useCallback(
    (canvas: HTMLCanvasElement | null) => {
      if (!canvas) return;
      canvasRef.current = canvas;

      workerRef.current?.terminate();
      setSampleTimes([]);
      setRenderStats(null);

      const worker = startRender(canvas, settings);
      worker.onmessage = (e: MessageEvent<WorkerOutMessage>) => {
        if (e.data.type === "sample") {
          const { sampleIndex, durationMs } = e.data;
          setRenderStats({ sampleIndex, durationMs });
          setSampleTimes((times) => [...times, durationMs]);
        }
      };
      workerRef.current = worker;
      setRunning(true);
    },
    [settings],
  );

  const handleStop = useCallback(() => {
    workerRef.current?.terminate();
    workerRef.current = null;
    setRunning(false);
  }, []);

  const handleRestart = useCallback(() => {
    setRenderKey((k) => k + 1);
  }, []);

  const handleSave = useCallback(() => {
    const canvas = canvasRef.current;
    if (!canvas) return;
    const a = document.createElement("a");
    a.href = canvas.toDataURL("image/png");
    a.download = "render.png";
    a.click();
  }, []);

  return {
    rendering: {
      canvasRefCallback,
      renderKey,
    },
    stats: {
      running,
      renderStats,
      sampleTimes,
    },
    handlers: {
      handleStop,
      handleRestart,
      handleSave,
    },
  };
};
