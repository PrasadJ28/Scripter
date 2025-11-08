fn main(){
     let r: &String;
    {
        let s = String::from("hello");
        r = &s;             
    }
    println!("{}", r);
}