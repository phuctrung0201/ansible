local wezterm = require("wezterm")

return {
	-- Font
	font_size = 16.0,

	-- Color scheme
	color_scheme = "Dracula",

	-- Tab bar
	enable_tab_bar = true,
	tab_bar_at_bottom = true,
	hide_tab_bar_if_only_one_tab = false,

	-- macOS: hide window title bar but keep resize controls
	window_decorations = "RESIZE",
	window_background_opacity = 0.85,
	macos_window_background_blur = 10,
}
