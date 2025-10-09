fn main() {
    let mut s=String::from ("");
    add_space(&mut s, 36);
    add_str(&mut s, "I");
    add_space(&mut s, 1);
    add_str(&mut s, "💚");
    add_str(&mut s, "\n");
    add_space(&mut s, 36);
    add_str(&mut s, "RUST.");
    add_str(&mut s, "\n");
    add_str(&mut s, "Most");
    add_space(&mut s, 12);
    add_str(&mut s, "crate");
    add_space(&mut s, 5);
    add_integer(&mut s, 306437968);
    add_space(&mut s, 11);
    add_str(&mut s, "and");
    add_space(&mut s, 5);
    add_str(&mut s, "latest");
    add_space(&mut s, 9);
    add_str(&mut s, "is");
    add_str(&mut s, "\n");
    add_space(&mut s, 5);
    add_str(&mut s, "downloaded");
    add_space(&mut s, 7);
    add_str(&mut s, "has");
    add_space(&mut s, 13);
    add_str(&mut s, "downloads");
    add_space(&mut s, 5);
    add_str(&mut s, "the");
    add_space(&mut s, 8);
    add_str(&mut s, "version");
    add_space(&mut s, 4);
    add_float(&mut s, 2.038);
    add_str(&mut s, ".");
    println!("{s}");
}
fn add_space(s:& mut String,n:i32)
{
    let mut i:i32=0;
    while i<n{
        s.push(' ');
        i+=1;
    }
}
fn add_str(s:& mut String, st: &str)
{
    s.push_str(st);
}
fn add_integer(s:& mut String, mut n: i32)
{
    let mut uc: i32;
    let mut inversn :i32;
    inversn=0;
    let mut tempchar: char;
    while n>0
    {
        uc=n%10;
        inversn=10*inversn+ uc;
        n/=10;
    }
    let mut contor3: i32;
    contor3=0;
    while inversn>0
    {
        uc=inversn%10;
        tempchar = (uc as u8 + b'0') as char;
         if contor3%3==0&&contor3!=0{
            s.push('_');
        }
        s.push(tempchar);
        inversn/=10;
        contor3+=1;
       
    }
}
fn add_float(s:&mut String, mut f: f32)
{
    let mut ct =0;
    while f !=f as i32 as f32{
        f*=10_f32;
        ct+=1;
    }
    let mut intf: i32=f as i32;
     let mut uc: i32;
    let mut inversf :i32;
    inversf=0;
    let mut tempchar: char;
    while intf >0
    {
        uc=intf %10;
        inversf=10*inversf+ uc;
        intf/=10;
    }
    let mut contor3: i32;
    contor3=0;
    while inversf>0
    {
        uc=inversf%10;
        tempchar = (uc as u8 + b'0') as char;
        s.push(tempchar);
        inversf/=10;
         contor3+=1;
         if contor3==ct{
            s.push('.');
        }
    }

}

