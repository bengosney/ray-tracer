export const humanDuration = (ms: number) => {
  if (Number.isNaN(ms)) return "NaN";
  if (ms <= 0) return "0ms";

  const totalSeconds = (ms / 1000) % 60;
  const seconds = Math.floor(totalSeconds);
  const minutes = Math.floor((ms / (1000 * 60)) % 60);
  const hours = Math.floor(ms / (1000 * 60 * 60));
  const showDecimal = seconds < 10 && minutes == 0;

  const parts = [];
  if (hours > 0) parts.push(`${hours}h`);
  if (minutes > 0) parts.push(`${minutes}m`);
  if (seconds > 0) parts.push(showDecimal ? `${totalSeconds.toFixed(1)}s` : `${seconds}s`);
  if (parts.length === 0) parts.push(`${ms.toFixed(0)}ms`);

  return parts.join(" ");
};
