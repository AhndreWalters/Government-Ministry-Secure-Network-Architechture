// breach.rs — Ministry of Education, Grenada
// Rust will REFUSE to compile if memory is unsafe.
// The compiler is the judge. There is no appeal.

fn main() {
    let guest_vlan: &str = "192.168.60.0/24";
    let condemned = guest_vlan; // ownership transferred
    // guest_vlan is now DEAD. You cannot use it again.
    // println!("{}", guest_vlan); ← this line will not compile
    println!("[TERMINATED] {} has been consumed", condemned);
}