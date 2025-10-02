

fn main() {
    let mut x = 2;
    let mut a=1;
    let mut b=1;
    while x < 101 {
        prime(x);
        x=x+1;
    }
    while a<100{
        while b<100{
            coprime(a,b);
            b=b+1;
        }
        a=a+1;
        b=1;
    }
    beri();
}
fn prime(x: i32) {
    let mut index = 2;
    let mut ok=1;
    while index <= x / 2 {
        if x % index == 0 {
            ok=0;
        }
        index = index + 1;
    }
    if ok==1
    {
        println!("{x}");
    }
    
}
fn cmmdc(mut a:i32,mut b:i32)->bool
{
        while b!=0{
            let r = a%b;
            a=b;
            b=r;
    }
     a==1
}
fn coprime(a:i32,b:i32){
    if cmmdc(a,b)
    {
        println!("{a},{b}");
    }
}
fn beri()
{
    let mut nr=99;
    while nr>1{
            println!("{nr} bottles of beer on the wall, {nr} bottles of beer");
            nr=nr-1;
            if nr!=1{
                println!("Take one down and pass it around. {nr} bottles of beer on the wall");
            }
            else{
                println!("Take one down and pass it around. {nr} bottle of beer on the wall");
            }
            
        }
    
}

