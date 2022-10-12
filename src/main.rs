mod submodule;

fn main() {
    println!("{}", submodule::submodule_testfile::VALUE);
    println!("{}", submodule::subsubmodule::subsubmodule_testfile::OTHER_VALUE);
}
