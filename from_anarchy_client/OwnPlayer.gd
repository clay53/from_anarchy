extends ARVROrigin

func _ready():
	print("Initializing ARVR Interface...")
	var arvr_interface = ARVRServer.find_interface("OpenVR")
	if arvr_interface and arvr_interface.initialize():
		var viewport = get_viewport()
		viewport.arvr = true
		#viewport.keep_3d_linear = true
		OS.vsync_enabled = false
		print("Initialized ARVR Interface")
	else:
		printerr("Failed to initialize ARVR Interface")
