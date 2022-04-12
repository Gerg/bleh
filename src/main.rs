extern crate getopts;
use getopts::Options;
use std::env;
use std::process::{exit, Command};

const AWS_BUCKET_URL: &str = "https://s3.amazonaws.com/bosh-core-stemcells";
const GCP_BUCKET_URL: &str = "https://storage.googleapis.com/bosh-core-stemcells";

const ALICLOUD: &str = "alicloud";
const ALICLOUD_PATH: &str = "alicloud-kvm";
const AWS: &str = "aws";
const AWS_PATH: &str = "aws-xen-hvm";
const AZURE: &str = "azure";
const AZURE_PATH: &str = "azure-hyperv";
const GCP: &str = "gcp";
const GCP_PATH: &str = "google-kvm";
const OPENSTACK: &str = "openstack";
const OPENSTACK_PATH: &str = "openstack-kvm";
const VCLOUD: &str = "vcloud";
const VCLOUD_PATH: &str = "vcloud-esxi";
const VSPHERE: &str = "vsphere";
const VSPHERE_PATH: &str = "vsphere-esxi";
const BOSH_LITE: &str = "bosh-lite";
const BOSH_LITE_PATH: &str = "warden-boshlite";

const DEFAULT_INF: &str = BOSH_LITE;

const INF_NAMES: [&str; 8] = [
    ALICLOUD, AWS, AZURE, GCP, OPENSTACK, VCLOUD, VSPHERE, BOSH_LITE,
];

const XENIAL: &str = "xenial";
const BIONIC: &str = "bionic";
const JAMMY: &str = "jammy";

const DEFAULT_OS: &str = BIONIC;

const OS_NAMES: [&str; 3] = [
    XENIAL, BIONIC, JAMMY
];

struct Infrastructure {
    name: &'static str,
    path: &'static str,
}

struct OperatingSystem {
    name: &'static str,
    bucket_url: &'static str
}

fn construct_inf(inf_option: Option<String>) -> Result<Infrastructure, String> {
    let inf: String = match inf_option {
        Some(x) => x,
        None => String::from(DEFAULT_INF),
    };
    return match &*inf {
        ALICLOUD => Ok(Infrastructure{name: ALICLOUD, path: ALICLOUD_PATH}),
        AWS => Ok(Infrastructure{name: AWS, path: AWS_PATH}),
        AZURE => Ok(Infrastructure{name: AZURE, path: AZURE_PATH}),
        GCP => Ok(Infrastructure{name: GCP, path: GCP_PATH}),
        OPENSTACK => Ok(Infrastructure{name: OPENSTACK, path: OPENSTACK_PATH}),
        VCLOUD => Ok(Infrastructure{name: VCLOUD, path: VCLOUD_PATH}),
        VSPHERE => Ok(Infrastructure{name: VSPHERE, path: VSPHERE_PATH}),
        BOSH_LITE => Ok(Infrastructure{name: BOSH_LITE, path: BOSH_LITE_PATH}),
        _ => Err(format!(
            "\"{name}\" is not a valid infrastructure.",
            name = inf
        )),
    };
}

fn construct_os(os_option: Option<String>) -> Result<OperatingSystem, String> {
    let os: String = match os_option {
        Some(x) => x,
        None => String::from(DEFAULT_OS),
    };

    return match &*os {
        XENIAL => Ok(OperatingSystem{ name: XENIAL, bucket_url: AWS_BUCKET_URL }),
        BIONIC => Ok(OperatingSystem{ name: BIONIC, bucket_url: GCP_BUCKET_URL }),
        JAMMY => Ok(OperatingSystem{ name: JAMMY, bucket_url: GCP_BUCKET_URL }),
        _ => Err(format!(
            "\"{name}\" is not a valid operating system.",
            name = os
        )),
    };
}

fn upload_stemcell(
    stemcell_version: &str,
    inf_option: Option<String>,
    os_option: Option<String>,
) -> Result<String, String> {
    let inf = construct_inf(inf_option)?;
    let os = construct_os(os_option)?;

    println!(
        "Attempting to upload stemcell for \"{os}\" operating system version \"{version}\" for infrastructure \"{inf}\".",
        os = os.name,
        version = stemcell_version,
        inf = inf.name
    );
    let stemcell_url = format!(
        "{bucket_url}/{v}/bosh-stemcell-{v}-{inf}-ubuntu-{os}-go_agent.tgz",
        bucket_url = os.bucket_url,
        v = stemcell_version,
        inf = inf.path,
        os = os.name
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
        "what infrastructure (options: {inf_names}; default: {default})",
        inf_names = INF_NAMES.join(", "),
        default = DEFAULT_INF
    );
    opts.optopt("i", "infrastructure", &*inf_help_text, "INFRASTRUCTURE");

    let os_help_text = format!(
        "what operating system version (options: {os_names}; default: {default})",
        os_names = OS_NAMES.join(", "),
        default = DEFAULT_OS
    );
    opts.optopt("o", "operating-system", &*os_help_text, "OPERATING_SYSTEM");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => {
            println!("{}", f);
            exit(1);
        }
    };

    let arg_missing = matches.free.is_empty();
    if matches.opt_present("h") || arg_missing {
        print_usage(&program, &opts);
        return;
    };

    let infrastructure = matches.opt_str("i");
    let operating_system = matches.opt_str("o");
    let stemcell_version = matches.free[0].clone();

    match upload_stemcell(&stemcell_version, infrastructure, operating_system) {
        Ok(_) => println!("Done!"),
        Err(e) => println!("{}", e),
    }
}
