<template>
  <div class="homepage">
    <!-- Homepage specific content goes here -->
    <h1>Welcome to Our Photoshop Clone App</h1>
    <div class="feature">
        <input type="file" @change="uploadImage" />
        <button @click="submitImage" class="button">Upload Image</button>
    </div>
    <!-- More content like introduction, features, etc. -->
  </div>
</template>

<script>
export default {
  data() {
    return {
      selectedImage: null,
    };
  },
  methods: {
    uploadImage(event) {
      this.selectedImage = event.target.files[0];
    },
    async submitImage() {
      if (!this.selectedImage) {
        alert("Please select an image to upload.");
        return;
      }
      const formData = new FormData();
      formData.append("image", this.selectedImage);

      try {
        await this.$axios.$post('/api/image/upload', formData, {
          headers: {
            'Content-Type': 'multipart/form-data',
          },
        });
        alert("Image uploaded successfully!");
      } catch (error) {
        console.error("Error uploading image:", error);
        alert("Failed to upload image.");
      }
    },
  },
}
</script>

<style>
/* Basic styling for the homepage */
.homepage {
    text-align: center;
    padding: 20px;
}

/* Styling for the main title */
h1 {
    color: #333;
    font-size: 2.5rem;
    margin-bottom: 20px;
}

/* Styling for feature sections or highlights */
.feature {
    background-color: #f4f4f4;
    padding: 15px;
    border-radius: 8px;
    margin: 15px 0;
}

/* Styling for buttons or links */
.button {
    background-color: #007bff;
    color: white;
    padding: 10px 20px;
    border: none;
    border-radius: 5px;
    cursor: pointer;
}

.button:hover {
    background-color: #0056b3;
}

/* Responsive design adjustments */
@media (max-width: 600px) {
    h1 {
        font-size: 2rem;
    }

    .feature {
        padding: 10px;
    }
}
</style>

