@tool
extends EditorPlugin

func _build() -> bool:
	var output = []
	var exit_code = OS.execute("cargo", ["build"], output, true, true);
	if exit_code != 0: printerr("".join(output))
	return exit_code == 0;
	
