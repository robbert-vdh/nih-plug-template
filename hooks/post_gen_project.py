from pathlib import Path

# NOTE: must match the names in the cookiecutter.editor_gui_framework and the
#       postfix of the editor_XXXX.rs files exactly
frameworks = ["iced", "vizia"]


def delete_editor_file(framework: str):
    editor_file = Path(f"src/editor_{framework}.rs")
    editor_file.unlink(missing_ok=True)


def rename_editor_file(framework: str):
    editor_file = Path(f"src/editor_{framework}.rs")
    editor_file.rename("src/editor.rs")


if __name__ == "__main__":
    for framework in frameworks:
        if framework == "{{ cookiecutter.editor_gui_framework }}":
            rename_editor_file(framework)
        else:
            delete_editor_file(framework)
