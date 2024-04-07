<script setup lang="ts">
const props = withDefaults(
  defineProps<{
    twelveHours?: boolean;
    secondsVisible?: boolean;
    blinking?: boolean;
    separator?: string;
  }>(),
  {
    twelveHours: false,
    secondsVisible: false,
    blinking: false,
    separator: ":",
  }
);

// TODO fix props change

const timeoutHandle = ref<NodeJS.Timeout>();
const timeString = ref<string>();

function updateText(currentTime: Date) {
  const hours = currentTime.getHours();
  const minutes = currentTime.getMinutes();
  const seconds = currentTime.getSeconds();

  const separatorVisible = props.blinking
    ? currentTime.getSeconds() % 2 === 0
    : true;

  let tmpTimeString = "";
  let suffix = "";
  const separatorFormatted = separatorVisible ? props.separator : " ";

  if (props.twelveHours) {
    tmpTimeString += (((hours + 11) % 12) + 1).toString().padStart(2, "0");
    if (hours >= 12) {
      suffix = " PM";
    } else {
      suffix = " AM";
    }
  } else {
    tmpTimeString += hours.toString().padStart(2, "0");
  }

  tmpTimeString += `${separatorFormatted}${minutes
    .toString()
    .padStart(2, "0")}`;

  if (props.secondsVisible) {
    tmpTimeString += `${separatorFormatted}${seconds
      .toString()
      .padStart(2, "0")}`;
  }

  tmpTimeString += suffix;

  timeString.value = tmpTimeString;
}

function tick() {
  // Clear already existing timeout (if it exists)
  if (timeoutHandle.value !== undefined) {
    clearTimeout(timeoutHandle.value);
  }

  const currentTime = new Date();
  updateText(currentTime);

  if (props.blinking || props.secondsVisible) {
    // If the seconds are visible or the cursor is blinking:
    // Update the text at the next full second
    timeoutHandle.value = setTimeout(
      tick,
      1000 - currentTime.getMilliseconds()
    );
  } else {
    // Update the text at the next full minute
    let delta =
      (60 - currentTime.getSeconds()) * 1000 - currentTime.getMilliseconds();
    // If the delta is > 1sec : Reduce timeout by 1 second to mitigate
    // a potential offset caused by the browser's timeout handling.
    // This causes another timeout & callback less than 1 sec before the actual target
    if (delta > 1000) {
      delta -= 1000;
    }
    timeoutHandle.value = setTimeout(tick, delta);
  }
}

onMounted(tick);
onUpdated(tick);
onUnmounted(() => {
  if (timeoutHandle.value !== undefined) {
    clearTimeout(timeoutHandle.value);
  }
});
</script>

<template>
  <span>
    {{ timeString }}
  </span>
</template>
