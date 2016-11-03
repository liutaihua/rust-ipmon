#### ip数据库parser

#### Usage

[dependencies.ipmon]
git = "https://github.com/liutaihua/rust-ipmon"

<pre><code>
extern crate ipmon;
use ipmon::{Locator};

fn main() {
    let locator = Locator::init("./ip.dat").unwrap();
    match locator.Find("202.96.209.5") {
        Ok(loc) => println!("location-> country: {}, region: {}, city: {}, isp: {}", loc.Country, loc.Region, loc.City, loc.Isp),
        Err(e) => println!("error: {:?}", e)
    }
}
</code></pre>
