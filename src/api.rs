// web api endpoints 

// POST /api/transform/rotate for a rotate tool.
// POST /api/filter/blur for applying a blur filter.

#[post("/upload" )]
fn upload_image() -> Result<HttpResponse, Error> {

    // Determine if this is a new image or a new layer for an existing image.
    // Process the image 
    // save the image metadata to the database 
    // Return a response with the image or layer identifier 
}