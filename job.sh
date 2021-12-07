#!/bin/bash
#SBATCH --job-name="Rust signal trap"
#SBATCH --time=0:5:00
#SBATCH --mem-per-cpu=100M
#SBATCH --cpus-per-task=1

##SBATCH --output=OUTPUT.out
##SBATCH --error=ERROR.out

#cargo clean
cargo run
