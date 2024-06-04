<script setup lang="ts">
const props = defineProps<{
  targetDate: Date;
  criticalTimeMs?: number;
}>();

const emit = defineEmits(["finished"]);

const displayText = ref<string>("");
const timeoutHandle = ref<NodeJS.Timeout>();

const criticalTimeDeltaMs =
  props.criticalTimeMs !== undefined ? props.criticalTimeMs : 120e3; // Default: 2min

function getCurrentStep(currentTimeDeltaMs: number): {
  nextStepTimeoutMs: number;
  displayTimeString: string;
} {
  if (currentTimeDeltaMs <= criticalTimeDeltaMs) {
    const minutes = Math.floor(currentTimeDeltaMs / 60e3);
    const seconds = Math.floor(currentTimeDeltaMs / 1000) % 60;

    const millisecondsOfCurrentTime = currentTimeDeltaMs % 1000;

    return {
      nextStepTimeoutMs: 1000 - millisecondsOfCurrentTime,
      displayTimeString: `${minutes.toString().padStart(2, "0")}:${seconds
        .toString()
        .padStart(2, "0")}`,
    };
  }

  const stringElements: string[] = [];
  const days = Math.floor(currentTimeDeltaMs / 86400e3);
  const hours = Math.floor(currentTimeDeltaMs / 3600e3) % 24;
  const minutes = Math.floor(currentTimeDeltaMs / 60e3) % 60;

  let nextStepTimeoutMs = 0;

  if (days !== 0) {
    stringElements.push(`${days}d`);
    nextStepTimeoutMs = days * 86400e3;
  }
  if (hours !== 0) {
    stringElements.push(`${hours}h`);
    nextStepTimeoutMs = hours * 3600e3;
  }
  if (minutes !== 0) {
    stringElements.push(`${minutes}min`);
    nextStepTimeoutMs = minutes * 60e3;
  }

  // If the delta is > 10sec : Reduce timeout by 1 second to mitigate
  // a potential offset caused by the browser's timeout handling.
  // This causes another timeout & callback less than 1 sec before the actual target
  if (nextStepTimeoutMs > 10e3) {
    nextStepTimeoutMs -= 1000;
  }

  console.log(nextStepTimeoutMs);

  return {
    nextStepTimeoutMs,
    displayTimeString: stringElements.join(" "),
  };
}

function initial_tick() {
  if (timeoutHandle.value !== undefined) {
    clearTimeout(timeoutHandle.value);
  }

  const now = Date.now();
  const timeDeltaMs = props.targetDate.valueOf() - now;

  if (timeDeltaMs <= 0) {
    displayText.value = "00:00";
    return;
  }

  tick();
}

function tick() {
  console.log("Tick");
  // Clear already existing timeout (if it exists)
  if (timeoutHandle.value !== undefined) {
    clearTimeout(timeoutHandle.value);
  }

  const now = Date.now();
  const timeDeltaMs = props.targetDate.valueOf() - now;

  console.log(`Delta: ${timeDeltaMs}`);

  if (timeDeltaMs <= 0) {
    displayText.value = "00:00";
    emit("finished");
    return;
  }

  console.log("aaa");
  let { nextStepTimeoutMs, displayTimeString } = getCurrentStep(timeDeltaMs);

  displayText.value = displayTimeString;

  const nextStepAbs = now + nextStepTimeoutMs;

  // if (
  //   now < props.targetDate.valueOf() - criticalTimeMs &&
  //   nextStepAbs >= props.targetDate.valueOf() - criticalTimeMs
  // ) {
  //   // Force switch to precise format
  //   console.log("kdsflj");
  //   nextStepMs = props.targetDate.valueOf() - criticalTimeMs;
  // }

  console.log(`Next step ms ${nextStepTimeoutMs}`);
  console.log(`Timeout target: ${new Date(nextStepAbs)}`);
  timeoutHandle.value = setTimeout(tick, nextStepTimeoutMs);
}

onMounted(initial_tick);
onUpdated(initial_tick);
onUnmounted(() => clearTimeout(timeoutHandle.value));
</script>

<template>
  <span>{{ displayText }}</span>
</template>
