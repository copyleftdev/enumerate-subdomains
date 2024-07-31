extern crate trust_dns_resolver;

use trust_dns_resolver::Resolver;
use trust_dns_resolver::config::*;

fn main() {
    let resolver = Resolver::new(ResolverConfig::default(), ResolverOpts::default()).unwrap();
    let domain = "example.com";
    let subdomains = vec!["www", "mail", "ftp", "test"];

    for sub in subdomains {
        let query = format!("{}.{}", sub, domain);
        match resolver.lookup_ip(query.as_str()) {
            Ok(response) => {
                for ip in response.iter() {
                    println!("{}: {}", query, ip);
                }
            }
            Err(_) => println!("{}: No record found", query),
        }
    }
}
