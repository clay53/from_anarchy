extends Node

export var websocket_url = "ws://127.0.0.1:8080"
export(NodePath) var server_path
onready var server = get_node(server_path)

var _client = WebSocketClient.new()

func _ready():
	_client.connect("connection_closed", self, "_closed")
	_client.connect("connection_error", self, "_closed")
	_client.connect("connection_established", self, "_connected")
	_client.connect("data_received", self, "_on_data")
	
	print("Attempting to connect...")
	var err = _client.connect_to_url(websocket_url)
	if err != OK:
		print("Unable to connect")
		set_process(false)

func _closed(was_clean = false):
	print("Closed, clean: ", was_clean)
	set_process(false)

func _connected(proto = ""):
	print("Connected with protocol: ", proto)
	_client.get_peer(1).put_packet(server.register_player())

func _on_data():
	var peer = _client.get_peer(1);
	print(peer.get_available_packet_count())
	var packet = peer.get_packet();
	server.parse_command(Array(packet))

func _process(delta):
	_client.poll()
