<template>
    <div>
      <input type="file" @change="selectFile" />
      <button @click="uploadImage" :disabled="!selectedFile">Upload Image</button>
      <div v-if="uploadResponse">
        <p>{{ uploadResponse.message }}</p>
        <img :src="uploadResponse.image_url" alt="Uploaded Image" />
      </div>
    </div>
  </template>
  
  <script>
  export default {
    data() {
      return {
        selectedFile: null,
        uploadResponse: null,
      };
    },
    methods: {
      selectFile(event) {
        this.selectedFile = event.target.files[0];
      },
      async uploadImage() {
        if (!this.selectedFile) {
          alert("Please select an image first.");
          return;
        }
        let formData = new FormData();
        formData.append("image", this.selectedFile);
        // Assuming your API endpoint expects the file under the key "image"
        try {
          const response = await this.$axios.$post('/api/image/upload', formData, {
            headers: {
              'Content-Type': 'multipart/form-data',
            },
          });
          this.uploadResponse = response;
          console.log('Upload successful:', response);
        } catch (error) {
          console.error('Error uploading image:', error);
          alert('Upload failed.');
        }
      },
    },
  }
  </script>
  