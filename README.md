# slurm-signal
You can run the program simply by calling `cargo run`.
Afterwards, pressing `ctrl+c` or sending `SIGINT` to the process
it'll output `"SIGINT trapped!` if it works correctly.

To create a slurm job, you can simply run
1. `sbatch job.sh` -> job_id,
2. wait for the job to start running.
3. `scancel --signal=INT <job_id>`.
If it works correctly the job should terminate gracefully and the
above mentioned output can be seen in the log file.

# Bug
I can confirm this runs flawlessly on my laptop and PC.
On UBELIX/slurm however, nothing is happening and the job gets killed
by the slurm manager.

It seems like the crate `signal-hook` is unable to trap `SIGINT` when
running as a slurm job.
