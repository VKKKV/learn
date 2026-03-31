from flask import Flask, jsonify, request

app = Flask(__name__)


@app.route("/upload", methods=["POST"])
def upload_file():
    if "file" not in request.files:
        return jsonify({"message": "No file part"}), 400

    file = request.files["file"]

    if file.filename == "":
        return jsonify({"message": "No selected file"}), 400

    # Save the file if needed
    # file.save(f"/path/to/save/{file.filename}")

    # Fixed JSON response
    response = {
        "message": f"File {file.filename} successfully uploaded",
        "detection_status": "Detection completed",
        "detection_result": [
            {"name": file.filename, "label": [2, 3], "attack_type": "access_control"}
        ],
    }

    return jsonify(response)


if __name__ == "__main__":
    app.run(host="0.0.0.0", port=8601)
