<script setup lang="ts">
const props = defineProps<{
  targetDate: Date;
  criticalTimeMs?: number;
  onFinish?: () => void;
}>();

const displayText = ref<string>("");
const timeoutHandle = ref<NodeJS.Timeout>();

const criticalTimeMs =
  props.criticalTimeMs !== undefined ? props.criticalTimeMs : 120e3; // Default: 2min

function getCurrentStep(currentTimeMs: number): {
  nextStepMs: number;
  displayTimeString: string;
} {
  if (currentTimeMs <= criticalTimeMs) {
    const minutesTotal = Math.floor(currentTimeMs / 60e3);
    const seconds = Math.floor(currentTimeMs / 1000) - minutesTotal * 60;

    return {
      nextStepMs: 1000 - currentTimeMs,
      displayTimeString: `${minutesTotal.toString().padStart(2, "0")}:${seconds
        .toString()
        .padStart(2, "0")}`,
    };
  }

  const stringElements: string[] = [];
  const days = Math.floor(currentTimeMs / 86400e3);
  const hours = Math.floor(currentTimeMs / 3600e3) % 24;
  const minutes = Math.floor(currentTimeMs / 60e3) % 60;

  let nextStepMs = 0;

  if (days !== 0) {
    stringElements.push(`${days}d`);
    nextStepMs = days * 86400e3;
  }
  if (hours !== 0) {
    stringElements.push(`${hours}h`);
    nextStepMs = hours * 3600e3;
  }
  if (minutes !== 0) {
    stringElements.push(`${minutes}min`);
    nextStepMs = minutes * 60e3;
  }

  // If the delta is > 1sec : Reduce timeout by 1 second to mitigate
  // a potential offset caused by the browser's timeout handling.
  // This causes another timeout & callback less than 1 sec before the actual target
  if (nextStepMs > 1000) {
    nextStepMs -= 1000;
  }

  return {
    nextStepMs: nextStepMs,
    displayTimeString: stringElements.join(" "),
  };
}

function tick() {
  console.log("Tick");
  // Clear already existing timeout (if it exists)
  if (timeoutHandle.value !== undefined) {
    clearTimeout(timeoutHandle.value);
  }

  const now = Date.now();
  const timeDeltaMs = props.targetDate.valueOf() - now;

  if (timeDeltaMs <= 0) {
    displayText.value = "00:00";
    if (props.onFinish !== undefined) {
      props.onFinish();
    }
    return;
  }

  let { nextStepMs, displayTimeString } = getCurrentStep(timeDeltaMs);

  displayText.value = displayTimeString;

  const nextStepAbs = now + nextStepMs;

  if (nextStepAbs >= props.targetDate.valueOf() - criticalTimeMs) {
    nextStepMs = now;
  }

  timeoutHandle.value = setTimeout(tick, nextStepMs);
}

onMounted(tick);
onUnmounted(() => clearTimeout(timeoutHandle.value));

watch(props.targetDate, tick);
</script>

<template>
  <span>{{ displayText }}</span>
</template>
