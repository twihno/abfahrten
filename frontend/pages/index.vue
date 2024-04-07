<script setup lang="ts">
import LiveClock from "../components/Timers/LiveClock.vue";
</script>

<template>
  <div>
    <p>Hello World</p>
    <p><LiveClock /></p>
    <!-- <p>
      <LiveTimer
        :target-date="new Date(Date.parse('2024-03-19T17:26:39+01:00'))"
      />
    </p> -->
  </div>
</template>
