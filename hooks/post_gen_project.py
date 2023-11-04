from shutil import rmtree, move
import os

# key value pairs from source location to destination location,
# also lets you rename files if you want
MOVE_PATHS = {
    {% if cookiecutter.__vizia == "True" %}"_editors/_editor-vizia.rs": "src/editor.rs"{% endif %}
}

DELETE_PATHS = [
    "_editors"
]

for src, dst in MOVE_PATHS.items():
    move(src, dst)

for path in DELETE_PATHS:
    if os.path.isdir(path):
        rmtree(path)
    else:
        os.remove(path)
