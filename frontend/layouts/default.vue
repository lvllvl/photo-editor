<template>
    <div class="app-container">
      <div class="toolbar">
        <!-- Toolbar components -->
      </div>
      <div class="main-content">
        <div class="top-menu">
          <!-- Top menu components -->
        </div>
        <div class="image-area">
          <!-- This is where your image will be displayed -->
          <Nuxt />
        </div>
      </div>
    </div>
  </template>
  
  <script>
  import Toolbar from '~/components/Toolbar.vue'
  // Import other components if needed
  
  export default {
    components: {
      Toolbar,
      // Other components
    }
  }
  </script>
  
  <style>
  .app-container {
    display: flex;
    height: 100vh;
    border: 2px solid #333; /* Narrow border around the screen */
  }
  
  .toolbar {
    width: 200px; /* Adjust width as needed */
    background-color: #f3f3f3; /* Background color for the toolbar */
    /* More styling for the toolbar */
  }
  
  .main-content {
    flex-grow: 1;
    display: flex;
    flex-direction: column;
  }
  
  .top-menu {
    background-color: #ddd; /* Background color for the top menu */
    padding: 10px;
    /* More styling for the top menu */
  }
  
  .image-area {
    flex-grow: 1;
    /* Styling for the area where the image will be displayed */
  }
  
  @media (max-width: 600px) {
    .toolbar {
      width: 100px; /* Narrower toolbar for smaller screens */
    }
  
    .top-menu {
      /* Adjustments for smaller screens */
    }
  
    .image-area {
      /* Adjustments for smaller screens */
    }
  }
  </style>
  