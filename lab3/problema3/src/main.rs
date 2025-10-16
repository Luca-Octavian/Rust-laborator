#[derive(Debug)]

enum Errors{
    Overflow32,
}

fn add_overflow(x:u32,y:u32)-> Result<u32,Errors>
{
    let a:u32 = u32::MAX;
    if x > a-y{
        Err(Errors::Overflow32)
    }
    else{
        Ok(x+y)
    }
}
fn multiply_overflow(x:u32,y:u32)-> Result<u32,Errors>
{
    let a:u32 = u32::MAX;
    if x > a/y{
        Err(Errors::Overflow32)
    }
    else{
        Ok(x*y)
    }
}
fn overflow_checker(x:u32,y:u32){
    let r1=add_overflow(x,y);
    if r1.is_err(){
        println!("{:?}",r1.err().unwrap());
    }
    else{
        println!("{:?}",r1.ok().unwrap());
    }
    let r2=multiply_overflow(x,y);
    if r2.is_err(){
        println!("{:?}",r2.err().unwrap());
    }
    else{
        println!("{:?}",r2.ok().unwrap());
    }
}
fn main() {
    overflow_checker(10,20);
    overflow_checker(2500000000,2500000000);
}
