import { humanDuration } from "./utils/time";

interface RenderStatsProps {
  sampleIndex: number;
  totalSamples: number;
  lastDurationMs: number;
  sampleTimes: number[];
}

function RenderStats({ sampleIndex, totalSamples, lastDurationMs, sampleTimes }: RenderStatsProps) {
  const avgSampleTime = sampleTimes.reduce((acc, curr) => acc + curr, 0) / sampleTimes.length;
  const totalTime = sampleTimes.reduce((acc, curr) => acc + curr, 0);
  const renderFinished = sampleIndex >= totalSamples;
  const samplesLeft = totalSamples - sampleIndex;
  const eta = samplesLeft * avgSampleTime;

  return (
    <p className="stats">
      <span>
        Sample: {sampleIndex}/{totalSamples}
      </span>
      {renderFinished ? (
        <span>Total: {humanDuration(totalTime)}</span>
      ) : (
        <span>Last: {humanDuration(lastDurationMs)}</span>
      )}
      <span>Avg: {humanDuration(avgSampleTime)}</span>
      {!renderFinished && <span>Eta: {humanDuration(eta)}</span>}
    </p>
  );
}

export default RenderStats;
