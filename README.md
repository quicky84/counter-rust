# counter-rust

Project is a client-server application.

Server must be started with 3 parameters: `port`, `nk` (number of kernels), and `timeout in secs`.

Server accepts the task identified with an `id` and given `difficulty` and counts up to `difficulty`;
if the computation is completed within the limit the number of milliseconds
it took to execute the computation is returned, otherwise a run-of-time message
is produced.

Client must be started with 4 parameters: `address`, `nt` (number of tasks), `min` and `max`.
Client generates `nt` numbers (`difficulties`) in the range `[min, max)`,
forms the same number of tasks and sends them to the server.

### Example
start server (in the `server` directory)

`cargo run 12345 20 10`

This commands to start a server on port `12345`,
use `20` cpu's (max possible will be used)
and sets execution timeout to `10` seconds.

start client (in the `client` directory)

`cargo run 127.0.0.1:12345 10 100000 100000000`

this will start a client which will fire `10` tasks at the server
to count to a random number between `100000` and `100000000`

You will see logging info on the server side:
1. what has been received,
2. how it has been parsed to a task,
3. when task has been completed,
4. what response has been sent back.

On the client side you will see the server's responses,
these responses will some order which depends how the server
executes the tasks.

### Also
You can connect in the terminal to the server as follows

```echo 2 1000000000 | ncat 127.0.0.1 12345```

this send the task with `id = 2` and `difficulty = 1000000000` (number of counts)
