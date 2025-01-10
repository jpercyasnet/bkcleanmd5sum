use std::path::{Path};
use std::io::{Write, BufRead, BufReader};
use std::fs::File;
use std::env;
use std::process::Command as stdCommand;

fn main()  {
    let mut bolok: bool = true;
    let bkrows_num: u64;
    let mut parm1dir = String::new();
    let args: Vec<_> = env::args().collect();
    if args.len() < 2 {
        println!(" no input parameter");
        bolok = false;
    } else {
        println!("The first argument is {}", args[1]);
        if args.len() > 2 {
            println!("Too many arguments");
            bolok = false;
        } else {
            if Path::new(&args[1]).exists() {
                println!("The first argument {} exists", args[1]);
                parm1dir = args[1].to_string();                    
                let outputy = stdCommand::new("wc")
                     .arg("-l")
                     .arg(&parm1dir)
                     .output()
                     .expect("failed to execute process");
                let strouty = String::from_utf8_lossy(&outputy.stdout);
                let vecout: Vec<&str> = strouty.split(" ").collect();
                let numlinesy: i64 = vecout[0].parse().unwrap_or(-9999);
                if numlinesy == -9999 {
                    println!("size of {} is invalid for wc -l command call", vecout[0]);
                    bolok = false;
                } else {
                    bkrows_num = numlinesy as u64;
                    if bkrows_num < 2 {
                        println!("size of {} is less than 2 for {}", bkrows_num, parm1dir);
                        bolok = false;
                    } else {
                        let filey = File::open(parm1dir.clone()).unwrap();
                        let mut readery = BufReader::new(filey);
                        let mut linebk = String::new();
                        let mut linenumy: u64 = 0;                            
                        loop {
                           match readery.read_line(&mut linebk) {
                              Ok(bytes_read) => {
                                 // EOF: save last file address to restart from this address for next run
                                 if bytes_read == 0 {
                                     println!("bytes_read == 0 for {}", parm1dir);
                                     bolok = false;
                                     break;
                                 }
                                 linenumy = linenumy + 1;
                                 if linenumy == 1 {
                                     if linebk.trim().to_string() == "refname|filename|dirname|filesize|filedate|md5sum|locations|notes".to_string() {
                                         println!("bk file is ok with size of {} rows", bkrows_num);
                                         break;
                                     } else {
                                         println!("first line of bk file is not valid: {}", linebk);
                                         bolok = false;
                                         break;
                                     }
                                 }         
                              }
                              Err(err) => {  
                                 println!("error of {} reading {}", err, parm1dir);
                                 bolok = false;
                                 break;
                              }
                           };
                        }
                    }
                }
            } else {
                println!("The first argument {} does not exist", args[1]);
                bolok = false;
            }
        }
    }
    if bolok {
        let mut outseq: u32 = 1;
        let mut goodout: String = format!("./goodout{:02}.excout", outseq);
        let mut badout: String = format!("./badout{:02}.excout", outseq);
        let mut nomd5out: String = format!("./nomd5out{:02}.excout", outseq);
        let mut quotout: String = format!("./quotout{:02}.excout", outseq);
        let mut errout: String = format!("./generrors{:02}.errout", outseq);
        loop {
               if !Path::new(&errout).exists() && !Path::new(&goodout).exists() && !Path::new(&badout).exists() {
                   break;
               } else {
                   outseq = outseq + 1;
                   goodout = format!("./goodout{:02}.excout", outseq);
                   badout = format!("./badout{:02}.neout", outseq);
                   nomd5out = format!("./nomd5out{:02}.excout", outseq);
                   quotout = format!("./quotout{:02}.excout", outseq);
                   errout = format!("./generrors{:02}.errout", outseq);
               }
        }          
        let mut goodfile = File::create(goodout).unwrap();
        let mut badfile = File::create(badout).unwrap();
        let mut nomd5file = File::create(nomd5out).unwrap();
        let mut quotfile = File::create(quotout).unwrap();
        let mut errfile = File::create(errout).unwrap();
        let filebk = File::open(parm1dir.clone()).unwrap();
        let mut readerbk = BufReader::new(filebk);
        let mut linebk = String::new();
        let mut linenumbk: u64 = 0;                            
        loop {
              match readerbk.read_line(&mut linebk) {
                Ok(bytes_read) => {
                  // EOF: save last file address to restart from this address for next run
                    if bytes_read == 0 {
                        let stroutput = format!("bytes_read == 0 for {}", parm1dir);
//                        println!("{}", stroutput);
                        writeln!(&mut errfile, "{}", stroutput).unwrap();
//                        bolbkend = true;
                        break;
                    }
                    linenumbk = linenumbk + 1;
                    if linenumbk > 1 {
                        let veclinez: Vec<&str> = linebk.split("|").collect();
                        if veclinez.len() < 6 {
                            if linebk.len() == 5 {
                                let stroutput = format!("invalid bk record {} line {}", linebk, linenumbk);
                                writeln!(&mut quotfile, "{}", stroutput).unwrap();
                            } else {
                                let stroutput = format!("invalid bk record {} line {}", linebk, linenumbk);
                                writeln!(&mut badfile, "{} length {}", stroutput, linebk.len()).unwrap();
                            }
                        } else {    
                            let mut bkmd5z: String = veclinez[5].to_string();
                            if bkmd5z.len() < 32 {
                                let stroutput = format!("md5sum less than 32 record {} line {}", linebk, linenumbk);
                                writeln!(&mut nomd5file, "{}", stroutput).unwrap();
                            }
                            if bkmd5z.len() > 32 {
                                if bkmd5z[..1].to_string() == '"'.to_string() {
                                    bkmd5z = bkmd5z[1..33].to_string();
                                }
                            }
                            let str7: String;
                            if veclinez.len() > 6 {
                                str7 = veclinez[6].to_string();
                            } else {
                                str7 = "".to_string();
                            }
                            let mut str8: String;
                            if veclinez.len() > 7 {
                                str8 = veclinez[7].to_string();
                            } else {
                                str8 = "".to_string();
                            }
                            if str8.len() < 5 {
                                str8 = "".to_string();
                            }
                            let linehdfmt = format!("{}|{}|{}|{}|{}|{}|{}|{}",
                            veclinez[0], veclinez[1], veclinez[2], veclinez[3], veclinez[4], bkmd5z, str7,str8);
                            writeln!(&mut goodfile, "{}", linehdfmt).unwrap();
                        }
                    }
                    linebk.clear();
                }
                Err(err) => {  
                   println!("error of {} reading {}", err, parm1dir);
                   break;
                }
              }
        }  
        println!("{} files", linenumbk);
    }
}
