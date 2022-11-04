# compile and run rs file, then delete it
# Usage: run_rs.sh <rs_file> 
rustc $1.rs 
./$1 
rm $1