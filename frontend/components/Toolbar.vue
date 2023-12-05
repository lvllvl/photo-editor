<template>
    <div class="toolbar bg-gray-200 w-20">
      <div v-for="tool in tools" :key="tool.id" class="toolbar-item">
        <!-- Render tool icon/button -->
        <button>{{ tool.name }}</button>
      </div>
    </div>
  </template>
  
  <script>
  export default {
    data() {
      return {
        tools: [
          { id: 1, name: 'Crop' },
          { id: 2, name: 'Rotate' },
          // ... more tools
        ]
      }
    }
  }
  </script>
  