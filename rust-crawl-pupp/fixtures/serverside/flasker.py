from flask import Flask, render_template, request, send_file, abort
from time import sleep

app = Flask(__name__)


@app.route("/form")
def form():
    sleep(5)
    return render_template("form.html")


@app.route("/download", methods=["GET"])
def download():
    return render_template("download.html")


@app.route("/download/<filename>", methods=["GET"])
def download_file(filename):
    try:
        import os

        print(f"Downloading file: {filename}")
        print(os.getcwd())
        # Assuming the files to be downloaded are in the 'downloadables' directory
        file_path = f"downloadables/{filename}"
        return send_file(file_path, as_attachment=True)
    except FileNotFoundError:
        abort(404)


@app.route("/data/", methods=["POST", "GET"])
def data():
    if request.method == "GET":
        return (
            f"The URL /data is accessed directly. Try going to '/form' to submit form"
        )
    if request.method == "POST":
        form_data = request.form
        return render_template("data.html", form_data=form_data)


app.run(host="localhost", port=5000)
