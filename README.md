# counter-rust

Project is a client-server application.

Client (is going to) generates a number, this number `n` together with an `id` form a task.
The task is to count to `n`;

Server must be started with 3 parameters: `port`, `n kernels`, and `timeout in secs`.

Server accepts the task and counts up to `n`;
if the computation is completed within the limit the number of milliseconds
it took to execute the computation is returned, otherwise a run-of-time message
is produced.
