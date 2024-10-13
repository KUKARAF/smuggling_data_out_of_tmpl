from flask import Flask, request
import os

app = Flask(__name__)

# Directory where uploaded files will be saved
UPLOAD_FOLDER = '/tmp/'
os.makedirs(UPLOAD_FOLDER, exist_ok=True)

app.config['UPLOAD_FOLDER'] = UPLOAD_FOLDER
app.config['MAX_CONTENT_LENGTH'] = 16 * 1024 * 1024  # Set a max file size limit (16MB example)

# Handle upload via POST and return a fallback for GET requests
@app.route('/', methods=['GET', 'POST'])
def upload_file():
    if request.method == 'POST':
        if 'file' not in request.files:
            return 'No file part in the request', 400

        file = request.files['file']

        # If no file is actually selected
        if file.filename == '':
            return 'No selected file', 400

        # Save the file to UPLOAD_FOLDER
        file_path = os.path.join(app.config['UPLOAD_FOLDER'], file.filename)
        file.save(file_path)

        return f'File {file.filename} uploaded successfully', 200

    # Fallback response for GET requests (or other requests that aren't POST)
    return '''
    <h1>File Upload</h1>
    <p>This endpoint only accepts POST requests to upload files.</p>
    <p><strong>Usage:</strong> Send a POST request with "file" in the form-data to upload the file.<br> 
       Example: Send a POST request to this endpoint using a file as "file" in the form.</p>
    ''', 200


if __name__ == '__main__':
    app.run(host='127.0.0.1', port=45678)
