
use target_info::Target;
fn main() {
println!("Target is {}-{}-{}", Target::arch(), Target::env(), Target::os());

}

