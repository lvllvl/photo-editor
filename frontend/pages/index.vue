<template>
  <div class="homepage">
    <div class="dropdown">
      <button class="dropbtn">File</button>
      <div class="dropdown-content">
        <a href="#" @click="triggerFileInput">Upload Image</a>
        <!-- Placeholder for more dropdown options -->
      </div>
    </div>
    <h1>Welcome to Our Photoshop Clone App</h1>
    <!-- Hidden file input for triggering upload -->
    <input type="file" @change="uploadImage" ref="fileInput" style="display: none;" />
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
    triggerFileInput() {
      this.$refs.fileInput.click();
    },
    async uploadImage(event) {
      this.selectedImage = event.target.files[0];
      
      // If no file was selected
      if (!this.selectedImage ) {
        alert( "Please select an image to upload." );
        return;
      }
      const formData = new FormData();
      // The key 'image' should match the expected key in the API endpoint
      formData.append( "image", this.selectedImage );

      try {
        const response = await this.$axios.$post('http://127.0.0.1:8080/image/upload', formData, {
          headers: {
            'Content-Type': 'multipart/form-data',
          },
        });
        // Handle response
        console.log( response );
        alert( "Image uploaded successfully!" );

      } catch (error) {
        console.error( "Error uploading image: ", error );
        alert( "Failed to upload image." ); 
      }
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

/* Dropdown button styling */
/* Dropdown Button */
.dropbtn {
  background-color: #007bff;
  color: white;
  padding: 10px 20px;
  border: none;
  cursor: pointer;
  border-radius: 5px;
}

/* The container <div> - needed to position the dropdown content */
.dropdown {
  position: relative;
  display: inline-block;
  top: 0;
  left: 0;
}

/* Dropdown Content (Hidden by default) */
.dropdown-content {
  display: none;
  position: absolute;
  background-color: #f9f9f9;
  min-width: 160px;
  box-shadow: 0px 8px 16px 0px rgba(0,0,0,0.2);
  z-index: 1;
}

/* Links inside the dropdown */
.dropdown-content a {
  color: black;
  padding: 12px 16px;
  text-decoration: none;
  display: block;
}

/* Change color of dropdown links on hover */
.dropdown-content a:hover {background-color: #f1f1f1}

/* Show the dropdown menu on hover */
.dropdown:hover .dropdown-content {display: block;}

/* Change the background color of the dropdown button when the dropdown content is shown */
.dropdown:hover .dropbtn {background-color: #0056b3;}
</style>

