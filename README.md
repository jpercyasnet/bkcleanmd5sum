# bkcleanmd5sum
clean up backup database md5sum field.

the dump file has rows with no md5sum and some have a " as a first character

need to run 

./target/release/bkcleanmd5sum bk2501adump.csv

this creates a file goodoutnn.excout
