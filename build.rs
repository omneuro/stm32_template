//Additoinal build tricks can go here but 
//all we need for the time being is a linker
//script from cortext-m-rt to include our memory fn main() 
// so the tools know how much RAM and Flash the chip has 
// and where to find them 

fn main(){
    //set the linker script to the one provided by cortext-m-rt
    println!("cargo:rustc-link-arg=-Tlink.x");
}
