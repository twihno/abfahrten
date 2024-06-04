<script setup lang="ts">
import LiveClock from "../components/Timers/LiveClock.vue";
import LiveTimer from "../components/Timers/LiveTimer.vue";
</script>

<template>
  <div>
    <p>Hello World</p>
    <p><LiveClock blinking /></p>
    <p>
      <LiveTimer
        :target-date="new Date(Date.parse('2024-05-30T01:54:30+02:00'))"
        @finished="() => console.log('YOLO')"
      />
    </p>
  </div>
</template>
