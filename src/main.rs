extern crate getopts;
use getopts::Options;
use std::env;
use std::process::{exit, Command};

const BUCKET_URL: &str = "https://s3.amazonaws.com/bosh-core-stemcells";

const ALICLOUD: &str = "alicloud";
const AWS: &str = "aws";
const AZURE: &str = "azure";
const GCP: &str = "gcp";
const OPENSTACK: &str = "openstack";
const VCLOUD: &str = "vcloud";
const VSPHERE: &str = "vsphere";
const BOSH_LITE: &str = "bosh-lite";

const INF_NAMES: [&str; 8] = [
    ALICLOUD, AWS, AZURE, GCP, OPENSTACK, VCLOUD, VSPHERE, BOSH_LITE,
];

fn inf_name(human_name: String) -> Result<String, String> {
    let computer_name = match &*human_name {
        ALICLOUD => Ok("alicloud-kvm".to_string()),
        AWS => Ok("aws-xen-hvm".to_string()),
        AZURE => Ok("azure-hyperv".to_string()),
        GCP => Ok("google-kvm".to_string()),
        OPENSTACK => Ok("openstack-kvm".to_string()),
        VCLOUD => Ok("vcloud-esxi".to_string()),
        VSPHERE => Ok("vsphere-esxi".to_string()),
        BOSH_LITE => Ok("warden-boshlite".to_string()),
        _ => Err(format!(
            "\"{name}\" is not a valid infrastructure",
            name = human_name
        )),
    };
    return computer_name;
}

fn upload_stemcell(
    stemcell_version: &str,
    infrastructure: Option<String>,
) -> Result<String, String> {
    let inf: String = match infrastructure {
        Some(x) => x,
        None => String::from("bosh-lite"),
    };
    println!(
        "Attempting to upload stemcell version \"{version}\" for infrastructure \"{inf}\"",
        version = stemcell_version,
        inf = inf
    );
    let computer_name = inf_name(inf)?;
    let stemcell_url = format!(
        "{bucket_url}/{v}/bosh-stemcell-{v}-{inf}-ubuntu-xenial-go_agent.tgz",
        bucket_url = BUCKET_URL,
        v = stemcell_version,
        inf = computer_name
    );

    println!("vvvvvvvvvvvvvvvvvvvvvvvvvvvvvv Shelling out to BOSH CLI vvvvvvvvvvvvvvvvvvvvvvvvvvvvvv\n\n");
    let mut cmd = Command::new("bosh")
        .args(&["upload-stemcell", &stemcell_url])
        .spawn()
        .unwrap();

    let status = cmd.wait();
    println!("\n\n^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ Done Shelling out to BOSH CLI ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^");
    return match status {
        Ok(v) => Ok(format!("Exited with status {:?}", v.code().unwrap())),
        Err(_) => Err("Failed to shell out to BOSH CLI".to_string()),
    };
}

fn print_usage(program: &str, opts: &Options) {
    let brief = format!("Usage: {} STEMCELL_VERSION [options]", program);
    println!("{}", opts.usage(&brief))
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optflag("h", "help", "print this help menu");
    let inf_help_text = format!(
        "what infrastructure (options: {inf_names}; default: bosh-lite)",
        inf_names = INF_NAMES.join(", ")
    );
    opts.optopt("i", "infrastructure", &*inf_help_text, "INFRASTRUCTURE");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => {
            println!("{}", f);
            exit(1);
        }
    };

    if matches.opt_present("h") {
        print_usage(&program, &opts);
    };

    let infrastructure = matches.opt_str("i");

    let stemcell_version = if !matches.free.is_empty() {
        matches.free[0].clone()
    } else {
        print_usage(&program, &opts);
        return;
    };

    match upload_stemcell(&stemcell_version, infrastructure) {
        Ok(_) => println!("Done!"),
        Err(e) => println!("{}", e),
    }
}
