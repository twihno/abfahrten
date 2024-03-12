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

const timeoutHandler = ref<NodeJS.Timeout>();
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

onUnmounted(() => {
  if (timeoutHandler.value !== undefined) {
    clearTimeout(timeoutHandler.value);
  }
});

onMounted(tick);

function tick() {
  if (timeoutHandler.value !== undefined) {
    clearTimeout(timeoutHandler.value);
  }

  const currentTime = new Date();
  updateText(currentTime);

  if (props.blinking || props.secondsVisible) {
    timeoutHandler.value = setTimeout(
      tick,
      1000 - currentTime.getMilliseconds()
    );
  } else {
    let delta =
      (60 - currentTime.getSeconds()) * 1000 - currentTime.getMilliseconds();
    if (delta > 1000) {
      delta -= 1000;
    }
    timeoutHandler.value = setTimeout(tick, delta);
  }
}
</script>

<template>
  {{ timeString }}
</template>
