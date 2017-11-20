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
start server

`cargo run 12345 2 10`

connect in terminals (there is no client yet)

in terminal 1 (a very long task)

```echo 2 1000000000 | ncat 127.0.0.1 12345```

**then** in terminal 2 (a quicker task)

```echo 1 13300 | ncat 127.0.0.1 12345```

The responses will arrive in *the following order*
* Task `1` will report back first with the execution time,
* Task `2` will never complete and will be halted due time-out
