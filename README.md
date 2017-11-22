# counter-rust

Project is a client-server application.

Client (is going to) generates a number, this number `n` together with an `id` form a task.
The task is to count to `n`;

Server must be started with 3 parameters: `port`, `n kernels`, and `timeout in secs`.

Server accepts the task and counts up to `n`;
if the computation is completed within the limit the number of milliseconds
it took to execute the computation is returned, otherwise a run-of-time message
is produced.

### Example
start server (in the `server` directory)

`cargo run 12345 20 10`

This commands to start a server on port 12345,
use 20 cpu's (max possible will be used)
and sets execution timeout to 10 seconds.

start client (in the `client` directory)

`cargo run 127.0.0.1:12345 10 100000 100000000`

this will start a client which will fire 10 tasks at the server
to count to a random number between 100000 and 100000000

You will see logging info on the server side:
1. what has been received,
2. how it was parsed to a task,
3. when task is completed,
4. what response was sent back.

On the client side you will see the server's responses,
these responses will some order which depends how the server
executes the tasks.

### Also
You can connect in the terminal to the server as follows

```echo 2 1000000000 | ncat 127.0.0.1 12345```

this echoes the task ik and task difficulty (number of counts)
