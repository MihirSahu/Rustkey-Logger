server_run:
	tmux new-session -d -s newsess
	tmux send-keys -t newsess "cargo run --bin server" Enter
	tmux attach -t newsess
client_run:
	tmux new-session -d -s newsess
	tmux send-keys -t newsess "cargo run --bin client" Enter
	tmux attach -t newsess
client_server_run:
	tmux new-session -d -s newsess
	tmux send-keys -t newsess "cargo run --bin server" Enter
	sleep 1
	tmux split-window -h -t newsess
	tmux send-keys -t newsess "cargo run --bin client" Enter
	tmux attach -t newsess